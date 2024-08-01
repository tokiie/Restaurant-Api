use log::{info, error};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::{Error, PgPool};
use uuid::Uuid;
// use chrono::Utc;

use crate::models::{restaurant_models::{PartialItem, PartialItemReturn, Table, Menu}, route_models::{FilterParams, Pagination}};

pub struct Database {
    pub pool: PgPool,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct NewItemRequest {
    pub quantity: i32,
    pub menu_id: Uuid,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BulkNewItemRequest {
    pub items: Vec<NewItemRequest>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateItemRequest {
    pub id: Uuid,
    pub quantity: Option<i32>,
    pub delivered_quantity: Option<i32>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BulkUpdateItemRequest {
    pub items: Vec<UpdateItemRequest>,
}

const MAX_ITEMS_LIMIT: usize = 100;

impl Database {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = PgPool::connect(database_url).await?;
        Ok(Database { pool })
    }

    pub async fn get_tables(&self) -> Result<Vec<Table>, Error> {
        let tables = sqlx::query_as!(
            Table,
            r#"
            SELECT
                id,
                name
            FROM tables
        "#)
            .fetch_all(&self.pool)
            .await?;

        Ok(tables)
    }

    pub async fn add_table(&self, name: String) -> Result<Table, Error> {
        let id = Uuid::new_v4();
        sqlx::query!(
            r#"
            INSERT INTO tables (id, name)
            VALUES ($1, $2)
            "#,
            id,
            name
        )
        .execute(&self.pool)
        .await?;

        Ok(Table { id, name })
    }


    pub async fn get_menu(&self) -> Result<Vec<Menu>, Error> {
        let menu = sqlx::query_as!(
            Menu,
            r#"
            SELECT *
            FROM Menu
        "#)
            .fetch_all(&self.pool)
            .await?;

        Ok(menu)
    }

    pub async fn add_menu(&self, name: String, price: Decimal, prep_time: i32) -> Result<Menu, Error> {
        let id = Uuid::new_v4();
        sqlx::query!(
            r#"
            INSERT INTO Menu (id, name, price, prep_time)
            VALUES ($1, $2, $3, $4)
            "#,
            id,
            name,
            price,
            prep_time
        )
        .execute(&self.pool)
        .await?;

        Ok(Menu {
            id,
            name,
            price,
            prep_time,
        })
    }

    pub async fn get_all_remaining_items_from_table(
        &self,
        tables_id: Uuid,
        pagination: Pagination,
        filters: FilterParams,
    ) -> Result<Vec<PartialItemReturn>, sqlx::Error> {
        let limit = pagination.limit.unwrap_or(10);
        let offset = pagination.offset.unwrap_or(0);

        let items = sqlx::query_as!(
            PartialItemReturn,
            r#"
            SELECT
                items.id,
                items.tables_id,
                items.menu_id,
                items.quantity,
                items.delivered_quantity,
                items.created_at,
                Menu.prep_time as prep_time
            FROM items
            LEFT JOIN Menu on menu_id = Menu.id
            WHERE tables_id = $1
              AND ($2::uuid IS NULL OR menu_id = $2)
              AND quantity > delivered_quantity
            LIMIT $3 OFFSET $4
            "#,
            tables_id,
            filters.menu_id,
            limit as i64,
            offset as i64
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(items)
    }

    pub async fn create_items(&self, tables_id: Uuid, new_items: Vec<NewItemRequest>) -> Result<Vec<PartialItem>, anyhow::Error> {
        let mut query = String::from("INSERT INTO items (id, tables_id, menu_id, quantity, delivered_quantity) VALUES ");
        let total_items = new_items.len();
        let mut placeholders = vec![];

        if total_items > MAX_ITEMS_LIMIT {
            return Err(anyhow::anyhow!("The number of items exceeds the limit of {}", MAX_ITEMS_LIMIT));
        }

        for i in 0..total_items {
            let start = i * 5 + 1;
            placeholders.push(format!("(${}, ${}, ${}, ${}, ${})", start, start + 1, start + 2, start + 3, start + 4));
        }

        query.push_str(&placeholders.join(", "));

        info!("Query: {}", query);

        let mut query_args = sqlx::query(&query);
        let mut created_items = Vec::with_capacity(total_items);

        for new_item in new_items {
            let id = Uuid::new_v4();

            query_args = query_args
                .bind(id)
                .bind(tables_id)
                .bind(new_item.menu_id)
                .bind(new_item.quantity)
                .bind(0);

            created_items.push(PartialItem {
                id,
                tables_id,
                menu_id: new_item.menu_id,
                quantity: new_item.quantity,
                delivered_quantity: 0
            });
        }

        query_args.execute(&self.pool).await.map_err(|err| {
            error!("Error creating items: {}", err);
            err
        })?;

        Ok(created_items)
    }

    pub async fn create_item(&self, tables_id: Uuid, new_item: NewItemRequest) -> Result<(), Error> {
        sqlx::query!(
            r#"
            INSERT INTO items (
                id, tables_id, menu_id, quantity,
                delivered_quantity
            ) VALUES (
                $1, $2, $3, $4,
                $5
            )
            "#,
            Uuid::new_v4(),
            tables_id,
            new_item.menu_id,
            new_item.quantity,
            0,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_item(&self, tables_id: Uuid, item_id: Uuid) -> Result<PartialItemReturn, Error> {
        let item = sqlx::query_as!(
            PartialItemReturn,
            r#"
            SELECT
                items.id,
                items.tables_id,
                items.menu_id,
                items.quantity,
                items.delivered_quantity,
                items.created_at,
                Menu.prep_time as prep_time
            FROM items
            LEFT JOIN Menu on menu_id = Menu.id
            WHERE items.tables_id = $1 AND items.id = $2
            "#,
            tables_id,
            item_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(item)
    }

    pub async fn delete_item(&self, tables_id: Uuid, item_id: Uuid) -> Result<bool, Error> {
        let result = sqlx::query!(
            r#"
            DELETE FROM items
            WHERE tables_id = $1 AND id = $2
            "#,
            tables_id,
            item_id
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn update_item(&self, item_id: Uuid, updated_item: UpdateItemRequest) -> Result<(), Error> {
        sqlx::query!(
            r#"
            UPDATE items
            SET
                quantity = COALESCE($1, quantity),
                delivered_quantity = COALESCE(delivered_quantity + $2, delivered_quantity)
            WHERE id = $3
            "#,
            updated_item.quantity,
            updated_item.delivered_quantity,
            item_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }


    pub async fn update_items(&self, items: Vec<UpdateItemRequest>) -> Result<usize, anyhow::Error> {
        if items.is_empty() {
            return Ok(0);
        } else if items.len() > MAX_ITEMS_LIMIT {
            return Err(anyhow::anyhow!("The number of items exceeds the limit of {}", MAX_ITEMS_LIMIT));
        }

        let mut cases_quantity = String::from("quantity = CASE ");
        let mut cases_delivered_quantity = String::from("delivered_quantity = CASE ");
        let mut ids: Vec<String> = vec![];

        for (i, _item) in items.iter().enumerate() {
            let id_placeholder = format!("${}", i * 3 + 1);
            let quantity_placeholder = format!("${}", i * 3 + 2);
            let delivered_quantity_placeholder = format!("${}", i * 3 + 3);

            cases_quantity.push_str(&format!("WHEN id = {} THEN {} ", id_placeholder, quantity_placeholder));
            cases_delivered_quantity.push_str(&format!("WHEN id = {} THEN {} ", id_placeholder, delivered_quantity_placeholder));
            ids.push(id_placeholder);
        }

        cases_quantity.push_str("ELSE quantity END");
        cases_delivered_quantity.push_str("ELSE delivered_quantity END");

        let query = format!(
            "UPDATE items SET {}, {} WHERE id IN ({})",
            cases_quantity,
            cases_delivered_quantity,
            ids.join(", ")
        );

        info!("Query: {}", query);

        let mut query_args = sqlx::query(&query);
        for item in items.iter() {
            query_args = query_args
                .bind(item.id)
                .bind(item.quantity)
                .bind(item.delivered_quantity);
        }

        let result = query_args.execute(&self.pool).await.map_err(|err| {
            error!("Error updating items: {}", err);
            anyhow::anyhow!(err)
        })?;

        Ok(result.rows_affected() as usize)
    }

}
