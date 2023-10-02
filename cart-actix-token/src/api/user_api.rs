            //// _____________ User Api____________  ////

////----------------------  START - Initial routes ----------------------------- ////

use actix_web::{HttpResponse, Responder, get, web::{self, Data, Json}, post, HttpRequest};
use mongodb::bson::oid::ObjectId;
use serde_json::json;

use crate::{repository::mongodb_repo::MongoRepo, model::user_model::{User, LoginUserSchema}};


//route handler function
#[get("/healthchecker")]
pub async fn initial() -> impl Responder {
    const MESSAGE: &str = "JWT Authentication in Rust using Actix-web, mongodb";
    HttpResponse::Ok().json(serde_json::json!({"status": "success", "message": MESSAGE}))
}

////----------------------  END - Initial routes ----------------------------- ////


////----------------------  START - User routes ----------------------------- ////


//user register handler function
#[post("/user-create")]
pub async fn register_user(db: Data<MongoRepo>, new_user: Json<User>) -> HttpResponse {
    let data = User {
        id: None,
        name: new_user.name.to_owned(),
        password: new_user.password.to_owned(),
        email: new_user.email.to_owned(),
        created_at: None
    };

    match db.register_user(data).await {
        Ok(_) => HttpResponse::Ok().json(json!({"status" : "success", "message" : "Registration Successfull"})),
        Err(error) =>  HttpResponse::ExpectationFailed().json(json!({"status" : "failed", "message" : error}))
    }
}

//user login handler function
#[post("/user-login")]
pub async fn login_user(user: web::Json<LoginUserSchema>, db: Data<MongoRepo>) -> HttpResponse {
    
    let user_details = db.login_user(user.into_inner());
    
    user_details.await

}

// //handler to get user total
// #[post("/get-user-total")]
// pub async fn get_cart(req: HttpRequest, db: Data<MongoRepo>) -> HttpResponse {
//     let auth = req.headers().get("Authorization");
//     let split: Vec<&str> = auth.unwrap().to_str().unwrap().split("Bearer").collect();    
//     let token = split[1].trim();

//     match db.get_user_total(token).await {
//         Ok(result) => HttpResponse::Ok().json(json!({"status" : "success", "result" : result})),
//         Err(error) =>  HttpResponse::ExpectationFailed().json(json!({"status" : "failed", "message" : error})),
//     }
// }

////----------------------  END - User routes ----------------------------- ////


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(initial)
    .service(register_user)
    .service(login_user);
}

////////----------------------  END  ----------------------------- ////////

