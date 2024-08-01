use serde::{Deserialize, Serialize};
use uuid::Uuid;
use super::restaurant_models::PartialItem;

#[derive(Debug, Deserialize)]
pub struct Pagination {
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

#[derive(Debug, Deserialize)]
pub struct FilterParams {
    pub menu_id: Option<Uuid>,
}

#[derive(Serialize)]
pub struct BulkNewItemResponse {
    pub items: Vec<PartialItem>,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub message: String,
}

#[derive(Serialize)]
pub struct SuccessResponse {
    pub message: String,
}