use actix_web::{HttpResponse, Responder, post, web};
use solana_sdk::{pubkey::Pubkey};
use spl_token::instruction::{initialize_mint};
use std::str::FromStr;
use crate::types::*;
use base64;
use serde_json;


#[post("/token/create")]
pub async fn create_token(
    req: web::Json<CreateTokenRequest>,
    state: web::Data<AppState>,
) -> impl Responder {
    let mintAuthority = match Pubkey::from_str(&req.mintAuthority) {
        Ok(pubkey) => pubkey,
        Err(_) => {
            return HttpResponse::BadRequest().json(serde_json::json!({
                "success": false,
                "error": "Invalid mint authority public key"
            }));
        }
    };
    let mint = match Pubkey::from_str(&req.mint) {
        Ok(pubkey) => pubkey,
        Err(_) => {
            return HttpResponse::BadRequest().json(serde_json::json!({
                "success": false,
                "error": "Invalid mint public key"
            }));
        }
    };
    let token_program_id = spl_token::id();
    let instruction = match initialize_mint(
        &token_program_id,
        &mint,
        &mintAuthority,
        None,
        req.decimals,
    ) {
        Ok(instr) => instr,
        Err(_) => {
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "success": false,
                "error": "Failed to create initialize mint instruction"
            }));
        }
    };
    let accounts = instruction
        .accounts
        .into_iter()
        .map(|account| AccountMetaData {
            pubkey: account.pubkey.to_string(),
            is_signer: account.is_signer,
            is_writable: account.is_writable,
        })
        .collect::<Vec<AccountMetaData>>();
    let instruction_data = base64::encode(&instruction.data);
    let response_data = CreateTokenResponse {
        program_id: token_program_id.to_string(),
        accounts,
        instruction_data,
    };
    HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: response_data,
    })
}

