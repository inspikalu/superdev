use actix_web::{App, HttpServer};
use std::env;
use std::sync::Arc;
use solana_client::rpc_client::RpcClient;
use actix_web::web;

mod types;
pub use types::AppState;

pub mod create_token;
pub mod gen_keypair;
pub mod mint_to_token;
pub mod message_sign;
pub mod verify_message;
pub mod send_sol;
pub mod send_token;

pub use create_token::*;
pub use gen_keypair::*;
pub use mint_to_token::*;
pub use message_sign::*;
pub use verify_message::*;
pub use send_sol::*;
pub use send_token::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let rpc_url = env::var("SOLANA_RPC_URL").unwrap_or_else(|_| {
        "https://api.devnet.solana.com".to_string()
    });

    
    let rpc_client = Arc::new(RpcClient::new(rpc_url.clone()));
    let state = web::Data::new(AppState { rpc_client });

    println!("Starting server on port {}", port);
    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(generate_keypair)
            .service(create_token)
            .service(mint_token)
            .service(sign_message)
            .service(verify_message)
            .service(send_sol)
            .service(send_token)
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}