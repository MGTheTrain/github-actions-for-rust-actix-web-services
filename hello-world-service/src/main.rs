use log::info;
use actix_web::{get, web, App, HttpServer, HttpResponse, Responder, Result};
use serde::Serialize;


#[derive(Serialize)]
struct HelloWorldResponse {
    message: String,
}

fn get_hello_world_message() -> HelloWorldResponse {
    HelloWorldResponse {
        message: String::from("Hello from Rust"),
    }
}

#[get("/api/v1/hws")]
async fn hello() -> Result<impl Responder> {
    let obj = get_hello_world_message();
    info!("Hello from Rust");
    Ok(web::Json(obj))
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    
    HttpServer::new(move || {
        App::new().
            service(hello)
    })
    .bind(("0.0.0.0", 80))? // Listening on all available network interfaces or addresses on the machine
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_hello_world_message() {
        let response = get_hello_world_message();
        assert_eq!(response.message, "Hello from Rust");
    }
}