use actix_web::{HttpResponse, Responder, post, web};
use solana_sdk::{pubkey::Pubkey};
use base64;
use bs58;
use crate::types::*;
use solana_sdk::signature::Signature;

#[post("/message/verify")]
pub async fn verify_message(
    req: web::Json<VerifyMessageRequest>,
) -> impl Responder {
    // Validate fields
    if req.message.is_empty() || req.signature.is_empty() || req.pubkey.is_empty() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "success": false,
            "error": "Missing required fields"
        }));
    }

    // Decode signature from base64
    let signature_bytes = match base64::decode(&req.signature) {
        Ok(bytes) => bytes,
        Err(_) => {
            return HttpResponse::BadRequest().json(serde_json::json!({
                "success": false,
                "error": "Invalid signature encoding"
            }));
        }
    };
    let signature = match Signature::try_from(signature_bytes.as_slice()) {
        Ok(sig) => sig,
        Err(_) => {
            return HttpResponse::BadRequest().json(serde_json::json!({
                "success": false,
                "error": "Invalid signature format"
            }));
        }
    };

    // Decode public key from base58
    let pubkey = match bs58::decode(&req.pubkey).into_vec() {
        Ok(bytes) => match Pubkey::try_from(bytes.as_slice()) {
            Ok(pk) => pk,
            Err(_) => {
                return HttpResponse::BadRequest().json(serde_json::json!({
                    "success": false,
                    "error": "Invalid public key format"
                }));
            }
        },
        Err(_) => {
            return HttpResponse::BadRequest().json(serde_json::json!({
                "success": false,
                "error": "Invalid public key encoding"
            }));
        }
    };

    // Verify the signature
    let valid = signature.verify(pubkey.as_ref(), req.message.as_bytes());

    let data = VerifyMessageData {
        valid,
        message: req.message.clone(),
        pubkey: req.pubkey.clone(),
    };
    HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "data": data
    }))
} 