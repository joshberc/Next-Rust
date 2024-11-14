#[macro_use] extern crate rocket;

use rocket::serde::{json::Json, Deserialize, Serialize};

// Struct to receive number input
#[derive(Deserialize)]
struct NumberInput {
    number: u64,
}

// Struct to return the result
#[derive(Serialize)]
struct PrimeCheckResult {
    is_prime: bool,
}

#[post("/api/prime-check", format = "json", data = "<input>")]
fn prime_check(input: Json<NumberInput>) -> Json<PrimeCheckResult> {
    Json(PrimeCheckResult {
        is_prime: is_prime(input.number),
    })
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

