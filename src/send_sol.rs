use actix_web::{HttpResponse, Responder, post, web};
use solana_sdk::{pubkey::Pubkey, system_instruction};
use std::str::FromStr;
use crate::types::*;
use base64;
use serde_json;

#[post("/send/sol")]
pub async fn send_sol(
    req: web::Json<SendSolRequest>,
) -> impl Responder {
    // Validate input
    if req.from.is_empty() || req.to.is_empty() || req.lamports == 0 {
        return HttpResponse::Ok().json(serde_json::json!({
            "success": false,
            "error": "Missing or invalid input fields"
        }));
    }
    let from_pubkey = match Pubkey::from_str(&req.from) {
        Ok(pk) => pk,
        Err(_) => {
            return HttpResponse::Ok().json(serde_json::json!({
                "success": false,
                "error": "Invalid sender address"
            }));
        }
    };
    let to_pubkey = match Pubkey::from_str(&req.to) {
        Ok(pk) => pk,
        Err(_) => {
            return HttpResponse::Ok().json(serde_json::json!({
                "success": false,
                "error": "Invalid recipient address"
            }));
        }
    };
    let instruction = system_instruction::transfer(&from_pubkey, &to_pubkey, req.lamports);
    let accounts = instruction.accounts.iter().map(|meta| meta.pubkey.to_string()).collect();
    let instruction_data = base64::encode(&instruction.data);
    let response_data = SendSolResponse {
        program_id: instruction.program_id.to_string(),
        accounts,
        instruction_data,
    };
    HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: response_data,
    })
} 