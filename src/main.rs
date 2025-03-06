

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};


#[actix_web::main]
 async fn main()-> std::io::Result<()> {
    // making a instance of httpServer
    HttpServer::new(|| {
        App::new()
            // we can use like this by passing the hello function to the service or
            .service(hello)

            .service(echo)
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