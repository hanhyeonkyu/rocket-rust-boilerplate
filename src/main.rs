#[macro_use]
extern crate rocket;

mod routes {
    pub mod welcome;
}
mod types {
    pub mod welcome;
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount(
        "/",
        routes![
            routes::welcome::landing,
            routes::welcome::echo,
            routes::welcome::health_check
        ],
    )
}
