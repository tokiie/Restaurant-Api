#[cfg(test)]
mod tests {
    use crate::{db::connection::{Database, NewItemRequest, UpdateItemRequest}, models::route_models::{FilterParams, Pagination}};

    use rust_decimal::Decimal;
    use sqlx::{postgres::PgPoolOptions, PgPool};
    use tokio;

    async fn cleanup_database(pool: &PgPool) -> Result<(), sqlx::Error> {
        sqlx::query("TRUNCATE TABLE items, Menu, Tables, Device RESTART IDENTITY CASCADE")
            .execute(pool)
            .await?;

        Ok(())
    }

    async fn setup_test_db() -> PgPool {
        let database_url = std::env::var("TEST_DATABASE_URL")
            .expect("TEST_DATABASE_URL must be set");
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await
            .expect("Failed to create test database pool");

        cleanup_database(&pool).await.expect("Failed to clean up database");

        sqlx::migrate!()
            .run(&pool)
            .await
            .expect("Failed to apply migrations");

        pool
    }


    #[tokio::test]
    async fn test_create_and_get_item() {
        let pool = setup_test_db().await;
        let db = Database { pool };

        let new_table = db.add_table("Test Table".to_string()).await.expect("Failed to add table");
        let tables_id = new_table.id;

        let new_menu = db.add_menu("Test Dish".to_string(), Decimal::new(1500, 2), 15).await.expect("Failed to add menu item");
        let menu_id = new_menu.id;

        let new_item = NewItemRequest {
            quantity: 2,
            menu_id,
        };
        db.create_item(tables_id, new_item).await.unwrap();

        let pagination = Pagination {
            limit: Some(10),
            offset: Some(0),
        };
        let filters = FilterParams { menu_id: None };

        let items = db.get_all_remaining_items_from_table(tables_id, pagination, filters).await.unwrap();

        assert_eq!(items.len(), 1);
        assert_eq!(items[0].tables_id, tables_id);
        assert_eq!(items[0].menu_id, menu_id);
        assert_eq!(items[0].quantity, 2);
        assert_eq!(items[0].delivered_quantity, 0);
    }

    #[tokio::test]
    async fn test_update_and_delete_item() {
        let pool = setup_test_db().await;
        let db = Database { pool };

        let new_table = db.add_table("Test Table".to_string()).await.expect("Failed to add table");
        let tables_id = new_table.id;

        let new_menu = db.add_menu("Test Dish".to_string(), Decimal::new(1500, 2), 15).await.expect("Failed to add menu item");
        let menu_id = new_menu.id;

        let new_item = NewItemRequest {
            quantity: 2,
            menu_id,
        };
        db.create_item(tables_id, new_item).await.unwrap();

        let pagination = Pagination {
            limit: Some(10),
            offset: Some(0),
        };
        let filters = FilterParams { menu_id: None };
        let items = db.get_all_remaining_items_from_table(tables_id, pagination, filters).await.unwrap();
        assert!(!items.is_empty(), "No items found for the table");
        let item_id = items[0].id;

        let update_request = UpdateItemRequest {
            id: item_id,
            quantity: Some(3),
            delivered_quantity: Some(1),
        };
        db.update_item(item_id, update_request).await.unwrap();

        let updated_item = db.get_item(tables_id, item_id).await.unwrap();
        assert_eq!(updated_item.quantity, 3);
        assert_eq!(updated_item.delivered_quantity, 1);

        let deleted = db.delete_item(tables_id, item_id).await.unwrap();
        assert!(deleted);

        let result = db.get_item(tables_id, item_id).await;
        assert!(result.is_err());
    }

}