use crate::types::welcome::*;

#[get("/")]
pub fn landing() -> &'static str {
  "Landing Page!"
}

#[post("/echo", data = "<input>")]
pub fn echo(input: rocket::serde::json::Json<IEcho>) -> String {
  format!("Welcome {}", input.name)
}

#[get("/health-check")]
pub fn health_check() -> &'static str {
  "I am healthy!"
}
