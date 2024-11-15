use rocket::serde::json::Json;
use rocket::{http::Status, response::status::Custom};

use crate::actions::{is_prime, generate_primes, median_primes};
use crate::models::{ErrorResponse, UpperLimit, NumberInput, MedianPrimesResult, PrimeCheckResult};

#[get("/")]
pub fn root() -> &'static str {
    "Welcome to the Rocket API!"
}

#[post("/api/median-primes", format = "json", data = "<input>")]
pub fn median_primes_route(input: Result<Json<UpperLimit>, rocket::serde::json::Error<'_>>) 
    -> Result<Json<MedianPrimesResult>, Custom<Json<ErrorResponse>>> 
{
    match input {
        Ok(valid_input) => {
            let primes = generate_primes(valid_input.n);
            let medians = median_primes(&primes);

            Ok(Json(MedianPrimesResult {
                median_primes: medians,
            }))
        }
        Err(_) => Err(Custom(
            Status::BadRequest,
            Json(ErrorResponse {
                error: String::from("Invalid input. Please provide a valid upper limit."),
            }),
        )),
    }
}

#[post("/api/prime-check", format = "json", data = "<input>")]
pub fn prime_check(input: Result<Json<NumberInput>, rocket::serde::json::Error<'_>>) 
    -> Result<Json<PrimeCheckResult>, Custom<Json<ErrorResponse>>> 
{
    match input {
        Ok(valid_input) => Ok(Json(PrimeCheckResult {
            is_prime: is_prime(valid_input.number),
        })),
        Err(_) => Err(Custom(
            Status::BadRequest,
            Json(ErrorResponse {
                error: String::from("Invalid input. Please provide a valid number."),
            }),
        )),
    }
}
