mod models;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_web::web::Json;
use serde::{Serialize, Deserialize};
use validator::Validate;
use crate::models::BuyPizzaRequest;

#[actix_web::main]
 async fn main()-> std::io::Result<()> {
    // making a instance of httpServer
    HttpServer::new(|| {
        App::new()
            // we can use like this by passing the hello function to the service or
            .service(hello)

            .service(echo)
            .service(buy_pizza)
            // by making routes to the new instance of the app
            // when some one call the /hey api it will response with the manual hello function
            .route("/hey",web::get().to(manual_hello))
    })
    .bind("127.0.0.1:8080")?
        .run()
        .await

}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}


#[post("/buypizza")]
async fn buy_pizza(body:Json<BuyPizzaRequest>) -> impl Responder {
    let is_valid = body.validate();
    match is_valid {
        Ok(_) => {
            let pizza_name = body.pizza_name.clone();
            HttpResponse::Ok().body("please enter a pizza name {pizza_name}")
        }
        Err(_) =>
            HttpResponse::Ok().body("pizza_name is required"),

    }
}



