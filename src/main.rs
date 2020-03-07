extern crate postgres;
extern crate dotenv;
extern crate chrono;

// 1.
pub mod user {
    tonic::include_proto!("user");
}

use tonic::{transport::Server};

// 1.
use user::{
    crud_server::{CrudServer},
};

extern crate uuid;

extern crate console;
use console::Style;

mod db_connection; // 2.

mod service;
use crate::service::User;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50055".parse().unwrap(); // 3.
    let user = User::default();

    let blue = Style::new()
        .blue();

    println!("\nRust gRPC Server ready at {}", blue.apply_to(addr)); // 4.

    Server::builder().add_service(CrudServer::new(user))
                     .serve(addr)
                     .await?;
    Ok(())
}
