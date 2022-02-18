#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

mod routes {
    pub mod user;
    pub mod welcome;
}
mod services {
    pub mod user;
}
mod types {
    pub mod user;
    pub mod welcome;
}
mod database;
use database::*;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/",
            routes![
                routes::welcome::landing,
                routes::welcome::echo,
                routes::welcome::health_check
            ],
        )
        .mount(
            "/user",
            routes![
                routes::user::get_users,
                routes::user::user_one,
                routes::user::make_user,
                routes::user::change_user
            ],
        )
}
