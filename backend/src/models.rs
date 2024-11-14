use rocket::serde::{Deserialize, Serialize};

// Struct to receive number input
#[derive(Deserialize)]
pub struct NumberInput {
    pub number: u64,
}

// Struct to return the result
#[derive(Serialize)]
pub struct PrimeCheckResult {
    pub is_prime: bool,
}

// Struct for error messages
#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}
