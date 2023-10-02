use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use chrono::prelude::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Cart {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    #[serde(rename = "_uid", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    pub product_name: String,
    pub price: f64,
    pub qty: f64,
    #[serde(rename = "_total", skip_serializing_if = "Option::is_none")]
    pub total: Option<f64>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,

}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateCart {
    pub qty: f64
}

