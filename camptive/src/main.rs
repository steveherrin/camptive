extern crate actix_web;
use actix_web::{server, App, HttpRequest};

fn hello(_req: &HttpRequest) -> &'static str {
    "{\"message\": \"Hello, world!\"}"
}

fn main() {
    let address = "127.0.0.1:8888";
    println!("Server running on http://{}", address);
    server::new(|| App::new().resource("/", |r| r.f(hello)))
        .bind(address)
        .unwrap()
        .run();
    println!("Shutting down...");
}
