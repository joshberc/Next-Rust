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

/// Represents the input for the median primes route.
/// Contains the upper limit `n` for finding prime numbers.
#[derive(Deserialize)]
pub struct UpperLimit {
    pub n: u64,
}

/// Represents the result for the median primes route.
/// Contains the median prime number(s).
#[derive(Serialize)]
pub struct MedianPrimesResult {
    pub median_primes: Vec<u64>,
}

/// Represents an error response for invalid input.
/// Contains an error message.
#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}
