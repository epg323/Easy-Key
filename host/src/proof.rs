use actix_web::web::{Json, Path};
use actix_web::{web, HttpResponse};
use host::keygen;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Response<T> {
    pub results: Vec<T>,
}

pub const APPLICATION_JSON: &str = "application/json";

#[derive(Debug, Deserialize, Serialize)]
pub struct Proof {
    pub phone_number: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProofRequest {
    pub phone_number: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProofResponse {
    pub receipt: String,
}

#[post("/gen-proof")]
pub async fn post(proof_req: Json<ProofRequest>) -> HttpResponse {
    let (receipt, public_address) = keygen(
        proof_req.password.to_string(),
        proof_req.phone_number.to_string(),
    );

    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(ProofResponse {
            receipt: public_address,
        })
}
