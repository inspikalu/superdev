use actix_web::{HttpResponse, Responder, post, web};
use solana_sdk::{pubkey::Pubkey, signer::keypair::Keypair, signer::Signer};
use spl_token::instruction::initialize_mint;
use std::str::FromStr;
use crate::types::*;

#[post("/keypair")]
pub async fn generate_keypair() -> impl Responder {
    let keypair = Keypair::new();
    let pubkey = keypair.pubkey().to_string();
    let secret = bs58::encode(keypair.to_bytes()).into_string();
    let data = GenKeyPairData { pubkey, secret };
    let response = ApiResponse { success: true, data };
    HttpResponse::Ok().json(response)
}
