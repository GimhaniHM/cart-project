            //// _____________ Todo Api____________  ////

////----------------------  START - Initial routes ----------------------------- ////

use actix_web::{HttpResponse, Responder, get, web::{self, Data, Json}, post, HttpRequest, put, delete};
use chrono::Utc;
use mongodb::bson::oid::ObjectId;
use serde_json::json;

use crate::{repository::mongodb_repo::MongoRepo, model::{user_model::{User, LoginUserSchema}, cart_model::{Cart, UpdateCart}}};


////----------------------  END - Initial routes ----------------------------- ////


////----------------------  START - User routes ----------------------------- ////

//user register handler function
#[post("/cart-create")]
pub async fn create_cart(req: HttpRequest, db: Data<MongoRepo>, new_cart: Json<Cart>) -> HttpResponse {

    let auth = req.headers().get("Authorization");
    let split: Vec<&str> = auth.unwrap().to_str().unwrap().split("Bearer").collect();
    let token = split[1].trim();

    println!("token: {:?}",token);

    let data = Cart {
        id: None,
        user_id: None,
        product_name: new_cart.product_name.to_owned(),
        price: new_cart.price,
        qty: new_cart.qty,
        total: None,
        created_at: None
    };

    match db.create_cart(token, data).await {
        Ok(list) => HttpResponse::Ok().json(json!({"status" : "success", "result" : list})),
        Err(err) => HttpResponse::ExpectationFailed().json(json!({"status" : "failed", "message" : err})),
    }
}

//Find all carts by id
#[get("/all-cart")]
pub async fn get_all_carts(req: HttpRequest, db: Data<MongoRepo>) -> HttpResponse {

    let auth = req.headers().get("Authorization");
    let split: Vec<&str> = auth.unwrap().to_str().unwrap().split("Bearer").collect();
    let token = split[1].trim();

    match db.list_all_carts_by_user(token).await {
        Ok(result) => result,
        Err(error) =>  HttpResponse::ExpectationFailed().json(json!({"status" : "failed", "message" : error})), 
    }
}

//handler to update the todo
#[put("/update-cart/{id}")]
pub async fn update_cart(req: HttpRequest, data: Json<UpdateCart>, id: web::Path<String>, db: Data<MongoRepo>) -> HttpResponse {

    let todo_id = id.into_inner();

    let auth = req.headers().get("Authorization");
    let split: Vec<&str> = auth.unwrap().to_str().unwrap().split("Bearer").collect();    
    let token = split[1].trim();

    let doc = UpdateCart {
        qty: data.qty.clone(),
    };

    match db.update_cart(token, doc, todo_id).await {
        Ok(result) => HttpResponse::Ok().json(json!({"result": result})),
        Err(err) => HttpResponse::Ok().json(err),
    }
}

//handler to delete the todo
#[delete("/delete-cart/{id}")]
pub async fn delete_cart(req: HttpRequest ,db: Data<MongoRepo>, id: web::Path<String>) -> HttpResponse {

    let delete_id = id.into_inner();
    let auth = req.headers().get("Authorization");
    let split: Vec<&str> = auth.unwrap().to_str().unwrap().split("Bearer").collect();    
    let token = split[1].trim();

    match db.delete_cart(token, delete_id).await {
        Ok(result) => HttpResponse::Ok().json(json!({"status" : "success", "result" : result})),
        Err(error) =>  HttpResponse::ExpectationFailed().json(json!({"status" : "failed", "message" : error})),
    }
}


//handler to get todo list
#[get("/get-cart/{id}")]
pub async fn get_cart(req: HttpRequest, db: Data<MongoRepo>, id: web::Path<String>) -> HttpResponse {
    let get_id = id.into_inner();
    let todo_id = ObjectId::parse_str(get_id).unwrap();
    let auth = req.headers().get("Authorization");
    let split: Vec<&str> = auth.unwrap().to_str().unwrap().split("Bearer").collect();    
    let token = split[1].trim();

    match db.finding_cart(token, &todo_id).await {
        Ok(result) => HttpResponse::Ok().json(json!({"status" : "success", "result" : result})),
        Err(error) =>  HttpResponse::ExpectationFailed().json(json!({"status" : "failed", "message" : error})),
    }
}



pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(create_cart)
    .service(get_all_carts)
    .service(update_cart)
    .service(delete_cart)
    .service(get_cart);
}