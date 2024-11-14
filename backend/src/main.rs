#[macro_use] extern crate rocket;
mod models;

use models::{NumberInput, PrimeCheckResult, ErrorResponse};
use rocket::serde::json::Json;
use rocket::{http::Status, response::status::Custom};

#[post("/api/prime-check", format = "json", data = "<input>")]
fn prime_check(input: Result<Json<NumberInput>, rocket::serde::json::Error<'_>>) 
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

#[get("/")]
fn root() -> &'static str {
    "Welcome to the Rocket API!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![root, prime_check])
}


// Helper function to check if a number is prime
fn is_prime(num: u64) -> bool {
    if num < 2 {
        return false;
    }
    for i in 2..=((num as f64).sqrt() as u64) {
        if num % i == 0 {
            return false;
        }
    }
    true
}

