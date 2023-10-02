use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use chrono::prelude::*;

//User structure
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub email: String,
    pub password: String,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>
}

//user login schema
#[derive(Debug, Deserialize)]
pub struct LoginUserSchema {
    pub email: String,
    pub password: String,
}

//Token claims structure
#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}

// //Total price structure for User
// #[derive(Debug, Serialize, Deserialize, Clone)]
// pub struct UserTotal {
//     #[serde(rename = "_uid", skip_serializing_if = "Option::is_none")]
//     pub id: Option<ObjectId>,
//     #[serde(rename = "_oid", skip_serializing_if = "Option::is_none")]
//     pub obj_id: Option<Vec<ObjectId>>,
//     #[serde(rename = "_tot", skip_serializing_if = "Option::is_none")]
//     pub total: Option<f64>
// }

