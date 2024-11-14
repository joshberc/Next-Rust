#[macro_use] extern crate rocket;

use rocket::serde::{json::Json, Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct InputData {
    value: String,
}

#[get("/")]
fn root() -> &'static str {
    "Welcome to the Rocket API!"
}

#[post("/api/data", format = "json", data = "<input>")]
fn handle_data(input: Json<InputData>) -> Json<String> {
    let response = format!("You sent: {}", input.value);
    Json(response)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![root, handle_data])
}

