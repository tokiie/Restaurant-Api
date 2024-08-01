
use std::sync::Arc;
use crate::db::connection::{BulkNewItemRequest, BulkUpdateItemRequest, Database};
use crate::{
    models::restaurant_models::PartialItem,
    models::route_models::{Pagination, FilterParams,  BulkNewItemResponse, ErrorResponse, SuccessResponse}
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json,
    Router,
};
use log::{info, error};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use uuid::Uuid;


//TODO: do filter on remaining items => not delivered items to a table
pub fn create_router(db: Arc<Database>) -> Router {
    return Router::new()
    .route("/tables/:tables_id/items", get(items_list).post(items_create).put(item_update))
    .route("/tables/:tables_id/items/:item_id", get(item_get).delete(item_delete))
    .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()))
    .with_state(db);
}


pub async fn items_list(
    Path(tables_id): Path<Uuid>,
    State(db): State<Arc<Database>>,
) -> impl IntoResponse {
    let pagination = Pagination { limit: None, offset: None };
    let filters = FilterParams { menu_id: None };

    match db.get_all_remaining_items_from_table(tables_id, pagination, filters).await {
        Ok(items) => {
            if items.is_empty() {
                info!("No items found for tables_id {}", tables_id);
                let error_response = ErrorResponse {
                    message: format!("No items found for table with id {}", tables_id),
                };
                (StatusCode::NOT_FOUND, Json(error_response)).into_response()
            } else {
                info!("Items found for tables_id {}", tables_id);
                Json(items).into_response()
            }
        }
        Err(e) => {
            let error_response = ErrorResponse {
                message: format!("Database error: {}", e),
            };
            error!("{}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)).into_response()
        }
    }
}

pub async fn items_create(
    Path(tables_id): Path<Uuid>,
    State(db): State<Arc<Database>>,
    Json(bulk_new_items): Json<BulkNewItemRequest>,
) -> impl IntoResponse {
    info!("Creating new items for table: {:?}", tables_id);

    match db.create_items(tables_id, bulk_new_items.items).await {
        Ok(created_items) => {
            if created_items.is_empty() {
                info!("No items found in request");
                let error_response = ErrorResponse {
                    message: format!("No items were created for table with id {}", tables_id),
                };
                (StatusCode::BAD_REQUEST, Json(error_response)).into_response()
            } else {
                info!("{} items added to table {}", created_items.len(), tables_id);
                let response_items: Vec<PartialItem> = created_items.into_iter().map(|item| PartialItem {
                    id: item.id,
                    tables_id: item.tables_id,
                    menu_id: item.menu_id,
                    quantity: item.quantity,
                    delivered_quantity: item.delivered_quantity,
                }).collect();

                (StatusCode::CREATED, Json(BulkNewItemResponse { items: response_items })).into_response()
            }
        }
        Err(e) => {
            error!("Failed to create items for table: {:?}. Error: {}", tables_id, e);
            let error_response = ErrorResponse {
                message: format!("Failed to create items: {}", e),
            };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)).into_response()
        }
    }
}

pub async fn item_get(
    Path((tables_id, item_id)): Path<(Uuid, Uuid)>,
    State(db): State<Arc<Database>>,
) -> impl IntoResponse {
    info!("Get item for tables {} and items {}", tables_id, item_id);
    match db.get_item(tables_id, item_id).await {
        Ok(item) => Json(item).into_response(),
        Err(e) => {
            error!("Failed to retrieve item for tables_id {} and item_id {}", tables_id, item_id);
            let error_response = ErrorResponse {
                message: format!("Failed to retrieve item: {}", e),
            };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)).into_response()
        },
    }
}

pub async fn item_delete(
    Path((tables_id, item_id)): Path<(Uuid, Uuid)>,
    State(db): State<Arc<Database>>,
) -> impl IntoResponse {
    info!("Trying to delete item {} for table {}", item_id, tables_id);
    match db.delete_item(tables_id, item_id).await {
        Ok(true) => StatusCode::NO_CONTENT.into_response(),
        Ok(false) => {
            let error_response = ErrorResponse {
                message: format!("Item with id {} not found in table {}", item_id, tables_id),
            };
            (StatusCode::NOT_FOUND, Json(error_response)).into_response()
        },
        Err(e) => {
            let error_response = ErrorResponse {
                message: format!("Failed to delete item: {}", e),
            };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)).into_response()
        },
    }
}

pub async fn item_update(
    State(db): State<Arc<Database>>,
    Json(bulk_updated_items): Json<BulkUpdateItemRequest>,
) -> impl IntoResponse {
    info!("Trying to update items");
    match db.update_items(bulk_updated_items.items).await {
        Ok(updated_count) => {
            if updated_count == 0 {
                info!("No updated items");
                let error_response = ErrorResponse {
                    message: "No items were updated".to_string(),
                };
                (StatusCode::NOT_FOUND, Json(error_response)).into_response()
            } else {
                info!("Successfuly updated {} item", updated_count);
                let success_response = SuccessResponse {
                    message: format!("{} items updated successfully", updated_count),
                };
                (StatusCode::OK, Json(success_response)).into_response()
            }
        },
        Err(e) => {
            error!("Failed to update items");
            let error_response = ErrorResponse {
                message: format!("Failed to update items: {}", e),
            };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)).into_response()
        },
    }
}