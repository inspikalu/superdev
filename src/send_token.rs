use actix_web::{HttpResponse, Responder, post, web};
use solana_sdk::{pubkey::Pubkey};
use spl_token::instruction::transfer;
use std::str::FromStr;
use crate::types::*;
use base64;

#[post("/send/token")]
pub async fn send_token(
    req: web::Json<SendTokenRequest>,
) -> impl Responder {
    // Validate input
    if req.destination.is_empty() || req.mint.is_empty() || req.owner.is_empty() || req.amount == 0 {
        return HttpResponse::Ok().json(serde_json::json!({
            "success": false,
            "error": "Missing or invalid input fields"
        }));
    }
    let destination_pubkey = match Pubkey::from_str(&req.destination) {
        Ok(pk) => pk,
        Err(_) => {
            return HttpResponse::Ok().json(serde_json::json!({
                "success": false,
                "error": "Invalid destination address"
            }));
        }
    };
    let mint_pubkey = match Pubkey::from_str(&req.mint) {
        Ok(pk) => pk,
        Err(_) => {
            return HttpResponse::Ok().json(serde_json::json!({
                "success": false,
                "error": "Invalid mint address"
            }));
        }
    };
    let owner_pubkey = match Pubkey::from_str(&req.owner) {
        Ok(pk) => pk,
        Err(_) => {
            return HttpResponse::Ok().json(serde_json::json!({
                "success": false,
                "error": "Invalid owner address"
            }));
        }
    };
    let token_program_id = spl_token::id();
    let instruction = match transfer(
        &token_program_id,
        &owner_pubkey,
        &destination_pubkey,
        &owner_pubkey,
        &[],
        req.amount,
    ) {
        Ok(instr) => instr,
        Err(_) => {
            return HttpResponse::Ok().json(serde_json::json!({
                "success": false,
                "error": "Failed to create transfer instruction"
            }));
        }
    };
    let accounts = instruction.accounts.iter().map(|meta| SendTokenAccountMeta {
        pubkey: meta.pubkey.to_string(),
        is_signer: meta.is_signer,
    }).collect();
    let instruction_data = base64::encode(&instruction.data);
    let response_data = SendTokenResponse {
        program_id: token_program_id.to_string(),
        accounts,
        instruction_data,
    };
    HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: response_data,
    })
} 