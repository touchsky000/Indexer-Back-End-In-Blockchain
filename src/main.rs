use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use actix_cors::Cors;
use serde::{Serialize, Deserialize};
use log::info;  // For logging
use env_logger::Env;  // To use env_logger
use actix_web::middleware::Logger;  // For request logging (similar to morgan)

#[derive(Serialize, Deserialize)]
struct HelloResponse {
    message: String,
}

#[derive(Deserialize, Serialize)]
struct MyQuery {
    name: String,
    age: Option<u32>, // Optional query parameter
}

async fn get(){
    
}

async fn hello(query: web::Query<MyQuery>) -> impl Responder {
    // Access query parameters
    info!("Received query parameters: name = {}, age = {:?}", query.name, query.age);
    
    HttpResponse::Ok().json(query.into_inner()) // Returning the query parameters as JSON
}

async fn print_body(body: web::Bytes) -> impl Responder {
    // Convert the Bytes to a String
    match String::from_utf8(body.to_vec()) {
        Ok(text) => {
            // Log the request body
            info!("Received request body: {}", text);
            HttpResponse::Ok().body("Request body received and logged.")
        }
        Err(_) => {
            HttpResponse::BadRequest().body("Invalid UTF-8 sequence in request body.")
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize the logger using env_logger
    env_logger::Builder::from_env(Env::default().default_filter_or("info"))
        .init();

    // Start the HTTP server with logging and CORS middleware
    let server = HttpServer::new(|| {
        App::new()
            // CORS middleware configuration (allow all origins)
            .wrap(Cors::permissive())
            // Enable logging middleware to log requests like `morgan`
            .wrap(Logger::default())  
            // Define routes
            .route("/", web::get().to(hello))
            .route("/log", web::post().to(print_body))
    })
    .bind("127.0.0.1:8080")?;

    info!("Server is running on port : 8080");
    
    server.run().await
}

//git 

