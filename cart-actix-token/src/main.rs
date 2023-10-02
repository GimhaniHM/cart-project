
use actix_web::{web::Data, HttpServer, App};

use crate::repository::mongodb_repo::MongoRepo;


mod model;
mod api;
//mod middleware;
mod repository;

use api::{user_api, cart_api};


#[actix_web::main]
async fn main() -> std::io::Result<()>{

    let db = MongoRepo::init().await;
    let db_data = Data::new(db);

    println!("🚀 Server started successfully");

    HttpServer::new(move || {

        App::new()
            .app_data(db_data.clone())
            .configure(user_api::config)
            .configure(cart_api::config)
    
        })
        .bind(("127.0.0.1", 8060))?
        .run()
        .await
}
