pub mod models;
pub mod schema;

use diesel::prelude::*;
use dotenv::dotenv;
use models::*;
use schema::users::dsl::*;
use schema::users::*;
use std::env;

fn establish_connection() -> SqliteConnection {
  dotenv().ok();

  let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  SqliteConnection::establish(&database_url)
    .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn get_users() -> Vec<User> {
  let connection = establish_connection();
  users
    .load::<User>(&connection)
    .expect("Error loading users")
}

pub fn get_user(search_id: i32) -> User {
  let connection = establish_connection();
  users
    .find(search_id)
    .first(&connection)
    .expect("Error loading users")
}

pub fn create_user(new_user: NewUser) {
  let connection = establish_connection();
  diesel::insert_into(users)
    .values((
      name.eq(new_user.name),
      age.eq(new_user.age),
      email.eq(new_user.email),
      pwd.eq(new_user.pwd),
    ))
    .execute(&connection)
    .expect("Error create new user");
}

pub fn update_user(key: i32, mod_user: NewUser) -> usize {
  let connection = establish_connection();
  diesel::update(users.find(key))
    .set((
      name.eq(mod_user.name),
      age.eq(mod_user.age),
      email.eq(mod_user.email),
      pwd.eq(mod_user.pwd),
    ))
    .execute(&connection)
    .expect("Can't Update User")
}
