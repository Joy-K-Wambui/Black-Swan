use std::env;
use actix_web::{web, App, HttpServer};
use dotenvy::dotenv;

mod db;
mod security;
mod models;
mod auth;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok(); // Load environment variables
    println!("DATABASE_URL: {:?}", env::var("DATABASE_URL"));


    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| {
        eprintln!("Error: DATABASE_URL not set");
        std::process::exit(1);
    });
    println!("Connecting to database at: {}", database_url);

    let pool = db::establish_connection().await.expect("Failed to connect to DB");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone())) // Pass DB pool
            .wrap(security::AuthMiddleware)
            .route("/register", web::post().to(auth::register_user)) // Register route
            .route("/login", web::post().to(auth::login)) // Login route
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}