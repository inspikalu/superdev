use actix_web::{HttpResponse, Responder, post, web};
use solana_sdk::{signer::keypair::Keypair, signer::Signer, pubkey::Pubkey};
use base64;
use bs58;
use crate::types::*;
use solana_sdk::signature::Signature;
use serde_json;

#[post("/message/sign")]
pub async fn sign_message(
    req: web::Json<SignMessageRequest>,
) -> impl Responder {
    if req.message.is_empty() || req.secret.is_empty() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "success": false,
            "error": "Missing required fields"
        }));
    }

    let secret_bytes = match bs58::decode(&req.secret).into_vec() {
        Ok(bytes) => bytes,
        Err(_) => {
            return HttpResponse::BadRequest().json(serde_json::json!({
                "success": false,
                "error": "Invalid secret key encoding"
            }));
        }
    };
    let keypair = match Keypair::from_bytes(&secret_bytes) {
        Ok(kp) => kp,
        Err(_) => {
            return HttpResponse::BadRequest().json(serde_json::json!({
                "success": false,
                "error": "Invalid secret key format"
            }));
        }
    };

    let signature = keypair.sign_message(req.message.as_bytes());
    let signature_b64 = base64::encode(signature.as_ref());
    let public_key_b58 = keypair.pubkey().to_string();

    let data = SignMessageData {
        signature: signature_b64,
        public_key: public_key_b58,
        message: req.message.clone(),
    };
    HttpResponse::Ok().json(ApiResponse {
        success: true,
        data,
    })
}

