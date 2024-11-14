use rocket::serde::json::Json;
use rocket::{http::Status, response::status::Custom};

use crate::models::{NumberInput, PrimeCheckResult, ErrorResponse};
use crate::actions::is_prime;

#[get("/")]
pub fn root() -> &'static str {
    "Welcome to the Rocket API!"
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
