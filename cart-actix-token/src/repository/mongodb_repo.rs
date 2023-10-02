use std::env;

use actix_web::{
    HttpResponse, 
    cookie::{time::Duration as ActixWebDuration, Cookie}, post
};
use chrono::{Utc, Duration};
use dotenv::dotenv;
use futures::StreamExt;
use jsonwebtoken::{encode, Header, EncodingKey, decode, DecodingKey, Validation, Algorithm};
use mongodb::{
    Client, 
    Collection, 
    results::{InsertOneResult, UpdateResult, DeleteResult}, 
    bson::{doc, extjson::de::Error, oid::ObjectId}};
use serde_json::json;

use crate::model::{user_model::{User, LoginUserSchema, TokenClaims}, cart_model::{Cart, UpdateCart}, response_model::ErrorResponse};

#[derive(Debug,Clone)]
pub struct MongoRepo {
    u_col: Collection<User>,
    cart_col: Collection<Cart>
}

impl MongoRepo {
    pub async fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };

        let client = Client::with_uri_str(uri).await.unwrap();
        let db = client.database("cart-db");
        let u_col: Collection<User> = db.collection("User");
        let cart_col: Collection<Cart> = db.collection("Cart");


        println!("✅ Database connected successfully");


        MongoRepo { 
            u_col,
            cart_col
        }
    } 

    ////----------------------  START - User handler function ----------------------------- ////

    //user find by email handler
    pub async fn find_by_email(&self, email: &String) -> Result<Option<User>, Error> {

        // let check_email = email;

        let user = self
            .u_col
            .find_one( doc! {"email" : email}, None)
            .await.ok()
            .expect("Error finding user");

        
        Ok(user)

    }

    //user find by email handler
    pub async fn find_by_email_pwd(&self, email: &String, pwd: &String) -> Result<Option<User>, Error> {

        // let check_email = email;

        let user = self
            .u_col
            .find_one( doc! {"email" : email, "password": pwd}, None)
            .await.ok()
            .expect("Error finding user");

        
        Ok(user)

    }

    //handler to create the user
    pub async fn register_user(&self, new_user: User) -> Result<InsertOneResult, ErrorResponse> {
        match self.find_by_email(&new_user.email.to_string()).await.unwrap(){
            Some(_x) => {
                Err(
                    ErrorResponse{
                        status: false,
                        message: "Email already exists".to_owned()
                    }
                )
            
            }

            None => {

                let doc = User {
                    id: None,
                    name: new_user.name,
                    email: new_user.email,
                    password: new_user.password,
                    created_at: None,
                }; 

                let user = self
                    .u_col
                    .insert_one(doc, None)
                    .await.ok()
                    .expect("Error creating user");

                    Ok(user)

            }
            
        }
    }

    //function for login user
    pub async fn login_user(&self, user: LoginUserSchema) -> HttpResponse {
        match self.find_by_email_pwd(&user.email, &user.password).await.unwrap() {
            Some(x) => {

                let jwt_secret = "secret".to_owned();

                let id = x.id.unwrap();  //Convert Option<ObjectId> to ObjectId using unwrap()

                let now = Utc::now();
                let iat = now.timestamp() as usize;
                
                let exp = (now + Duration::minutes(60)).timestamp() as usize;
                let claims: TokenClaims = TokenClaims {
                    sub: id.to_string(),
                    exp,
                    iat,
                };
                

                let token = encode(
                    &Header::default(),
                    &claims,
                    &EncodingKey::from_secret(jwt_secret.as_ref()),
                )
                .unwrap();

                let cookie = Cookie::build("token", token.to_owned())
                    .path("/")
                    .max_age(ActixWebDuration::new(60 * 60, 0))
                    .http_only(true)
                    .finish();

                // Ok(LoginResponse {
                //     status: true,
                //     token,
                //     message: "You have successfully logged in.".to_string(),
                // })
                

                HttpResponse::Ok()
                    .cookie(cookie)
                    .json(json!({"status" :  "success", "token": token}))
            },

            None => {
                return HttpResponse::BadRequest()
                .json(ErrorResponse{
                    status: false,
                    message: "Invalid username or password".to_owned()
                })
            }
        }

    }

    ////----------------------  END - User handler function ----------------------------- ////



    ////----------------------  START - Todo List handler function ----------------------------- ////

    //handler to validate the user
    pub async fn validate_user(&self, token: &str) -> Result<Option<User>, HttpResponse>{
        let secret_key = "secret".to_owned();
    
        let var = secret_key;
        let key = var.as_bytes();
        let decode = decode::<TokenClaims>(
            token,
            &DecodingKey::from_secret(key),
            &Validation::new(Algorithm::HS256),
        ); 

        println!("decode: {:?}", decode);

        match decode {
            Ok(decoded) => {

                println!("object_id{:?}", decoded.claims.sub.to_owned());

                let id = decoded.claims.sub;

                let bson_id = ObjectId::parse_str(id).unwrap(); //used to convert <String> to <bjectId>

                let user = self
                    .u_col
                    .find_one( doc! {"_id" : bson_id }, None)
                    .await.ok()
                    .expect("Error finding user");

                println!("{:?}", user);
        
                Ok(user)

            }
            Err(_) => Err(
                //HttpResponse::BadRequest().json(json!({"status" :  "fail", "message": "Invalid token"})))
                HttpResponse::BadRequest().json(ErrorResponse{
                    status: false,
                    message: "Invalid token".to_owned()
                }))
            
        }
    }

    //create todo list
    pub async fn create_cart(&self, token: &str, new_cart: Cart) -> Result<InsertOneResult, ErrorResponse> {
        match self.validate_user(token).await.unwrap(){
            Some(x) => {
                let user_id = x.id.unwrap().to_string();

                let data = Cart {
                    id: None,
                    user_id: Some(user_id),
                    product_name: new_cart.product_name,
                    price: new_cart.price,
                    qty: new_cart.qty,
                    total: (Some(new_cart.price*new_cart.qty)),
                    created_at: Some(Utc::now())
                };

                // // Create a HashMap and insert the new_todo into it
                // let mut todo_list = HashMap::new();
                // todo_list.insert(user_id, new_data);

                // let doc = TodoList {
                //     list: todo_list,
                // };

                let todo_doc = self 
                    .cart_col
                    .insert_one(data, None)
                    .await
                    .ok()
                    .expect("Error creating cart");

                println!("{:?}", todo_doc);
                Ok(todo_doc)
            },
            None => {
                Err(ErrorResponse {
                    status: false,
                    message: "Not found user".to_string(),
                })
            }
        }
    }


//handler to list all the Todos specified to User
    pub async fn list_all_carts_by_user(&self, token: &str) -> Result<HttpResponse, ErrorResponse> {
        match self.validate_user(token).await.unwrap(){
            Some(x) => {

                let user_id = x.id.unwrap().to_string();

                let doc = doc! {
                    "_uid": user_id
                };

                let mut cart_doc = self 
                    .cart_col
                    .find(doc, None)
                    .await
                    .ok()
                    .expect("Error geting cart data");

                    let mut cart_vec = Vec::new();

                    let mut user_total: f64 = 0.0;

                    while let Some(doc) = cart_doc.next().await {

                        match doc {
                            Ok(data) => {
                                user_total = user_total + data.total.unwrap();
                                cart_vec.push(data)
                            },
                            Err(err) => {
                                eprintln!("Error finding cart: {:?}", err)
                            },
                        }
                    }    

                println!("{:?}", user_total);
                //println!("{:?}", cart_doc);
               // Ok(cart_vec)
               Ok(HttpResponse::Ok().json(json!({"status" : "success", "result" : cart_vec, "Total": user_total})))
            },
            None => {
                Err(ErrorResponse {
                    status: false,
                    message: "Not found user".to_string(),
                })
            }
        }
    }

    //handler to update cart
    pub async fn update_cart(&self, token: &str, cart_data: UpdateCart, cart_id: String ) -> Result<UpdateResult, ErrorResponse> {
        match self.validate_user(token).await.unwrap(){
            Some(_x) => {

                //let _user_id = x.id.unwrap().to_string();

                let cart_id = ObjectId::parse_str(cart_id).unwrap();

                match self.finding_cart(&token, &cart_id).await.unwrap() {
                    Some(data) => {

                        let filter = doc! {"_id": cart_id};

                        let update_total = data.price * cart_data.qty;
    
                        let new_doc = doc! {
                            "$set":
                                {
                                    "qty": cart_data.qty,
                                    "_total": update_total
                                },
                        };
                        let updated_doc = self
                            .cart_col
                            .update_one(filter, new_doc, None)
                            .await
                            .ok()
                            .expect("Error updating cart");
                
                        Ok(updated_doc)
                    },
                    None => {
                        return Err(ErrorResponse {
                                message: "Todo Not found".to_owned(),
                                status: false
                        })
                    }
                }
            },
            None => {
                Err(ErrorResponse {
                    status: false,
                    message: "Not found user".to_string(),
                })
            }
        }
    }

//handler for finding cart
pub async fn finding_cart(&self, token: &str, cart_id: &ObjectId) -> Result<Option<Cart>, ErrorResponse> {
    match self.validate_user(token).await.unwrap(){
        Some(x) => {

            let user_id = x.id.unwrap().to_string();

            let todo = self
            .cart_col
            .find_one(doc! {
                "_id" : cart_id,
                "_uid" : user_id
            }, None)
            .await.ok()
            .expect("Error finding cart");

            Ok(todo)
        },
        None => Err(ErrorResponse {
            status: false,
            message: "Not found user".to_string(),
        })
    }
    
}

//handler for delete the todo list
pub async fn delete_cart(&self, token: &str, cart_id: String) -> Result<DeleteResult, ErrorResponse> {
    match self.validate_user(token).await.unwrap(){
        Some(x) => {

            let _user_id = x.id.unwrap().to_string();

            let cart_id = ObjectId::parse_str(&cart_id).unwrap();

            match self.finding_cart(&token, &cart_id).await.unwrap() {
                Some(_) => {

                    let filter = doc! {"_id": cart_id};

                    let delete_doc = self
                            .cart_col
                            .delete_one(filter, None)
                            .await
                            .ok()
                            .expect("Error deleting todos");
            
                    Ok(delete_doc)
                },
                None => {
                    return Err(ErrorResponse {
                            message: "Todo Not found".to_owned(),
                            status: false
                    })
                }
            }
        },
        None => Err(ErrorResponse {
            status: false,
            message: "Not found user".to_string(),
        })
    }
}

// //handler to get user total 
// pub async fn get_user_total(&self, token: &str) -> Result<UpdateResult, ErrorResponse> {
//     match self.validate_user(token).await.unwrap(){
//         Some(_x) => {

//             let _user_id = x.id.unwrap().to_string();

//             //let cart_id = ObjectId::parse_str(cart_id).unwrap();

//             match self.finding_cart(&token, &cart_id).await.unwrap() {
//                 Some(data) => {

//                     let filter = doc! {"_id": cart_id};

//                     let update_total = data.price * cart_data.qty;

//                     let new_doc = doc! {
//                         "$set":
//                             {
//                                 "qty": cart_data.qty,
//                                 "_total": update_total
//                             },
//                     };
//                     let updated_doc = self
//                         .cart_col
//                         .update_one(filter, new_doc, None)
//                         .await
//                         .ok()
//                         .expect("Error updating cart");
            
//                     Ok(updated_doc)
//                 },
//                 None => {
//                     return Err(ErrorResponse {
//                             message: "Todo Not found".to_owned(),
//                             status: false
//                     })
//                 }
//             }
//         },
//         None => {
//             Err(ErrorResponse {
//                 status: false,
//                 message: "Not found user".to_string(),
//             })
//         }
//     }
// }



}