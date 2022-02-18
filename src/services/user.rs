use crate::database;
use crate::types::user::*;

pub async fn get_users() -> IUserAll {
  let dt = database::get_users();
  let ret = IUserAll {
    rt: true,
    dt,
    mg: "success".to_string(),
  };
  return ret;
}

pub async fn user_one(id: i32) -> IUserOne {
  let dt = database::get_user(id);
  let ret = IUserOne {
    rt: true,
    dt,
    mg: "success".to_string(),
  };
  return ret;
}

pub async fn make_user(body: rocket::serde::json::Json<IMakeUserReq>) -> IMakeUserRes {
  let new_user = database::models::NewUser {
    name: &body.name,
    age: &body.age,
    email: &body.email,
    pwd: &body.pwd,
  };
  database::create_user(new_user);
  let ret = IMakeUserRes {
    rt: true,
    mg: "success".to_string(),
  };
  return ret;
}

pub async fn change_user(
  id: i32,
  body: rocket::serde::json::Json<IChangeUserReq>,
) -> IChangeUserRes {
  let mod_user = database::models::NewUser {
    name: &body.name,
    age: &body.age,
    email: &body.email,
    pwd: &body.pwd,
  };
  let dt = database::update_user(id, mod_user);
  let ret = IChangeUserRes {
    rt: true,
    dt,
    mg: "success".to_string(),
  };
  return ret;
}
