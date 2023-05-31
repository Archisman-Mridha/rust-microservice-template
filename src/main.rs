#![allow(non_snake_case)]

use actix_web::{HttpServer, App, web};

async fn generalHTTPHandler( ) -> &'static str {
  return "Hello world";
}

#[actix_web::main]
async fn main( ) -> std::io::Result<( )> {
  println!("🚀 Starting ActixWeb HTTP server!");

  HttpServer::new(
    | | {
      return App::new( )
        .route("/", web::get( ).to(generalHTTPHandler));
    }
  )
    .bind(("0.0.0.0", 8000))?
    .run( )
    .await?;

  return Ok(( ));
}