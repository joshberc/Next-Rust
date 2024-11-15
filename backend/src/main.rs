#[macro_use] extern crate rocket;

mod actions;
mod models;
mod routes;

use routes::{root, prime_check, median_primes_route};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![root, prime_check, median_primes_route])
}


