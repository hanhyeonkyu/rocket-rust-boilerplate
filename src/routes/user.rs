use crate::services;
use crate::types::user::*;

#[get("/all")]
pub async fn get_users() -> rocket::serde::json::Json<IUserAll> {
  let ret = services::user::get_users().await;
  return rocket::serde::json::Json(ret);
}

#[get("/<customer_id>")]
pub async fn user_one(customer_id: i32) -> rocket::serde::json::Json<IUserOne> {
  let ret = services::user::user_one(customer_id).await;
  return rocket::serde::json::Json(ret);
}

#[post("/post", data = "<body>")]
pub async fn make_user(
  body: rocket::serde::json::Json<IMakeUserReq>,
) -> rocket::serde::json::Json<IMakeUserRes> {
  let ret = services::user::make_user(body).await;
  return rocket::serde::json::Json(ret);
}

#[put("/<customer_id>/put", data = "<body>")]
pub async fn change_user(
  customer_id: i32,
  body: rocket::serde::json::Json<IChangeUserReq>,
) -> rocket::serde::json::Json<IChangeUserRes> {
  let ret = services::user::change_user(customer_id, body).await;
  return rocket::serde::json::Json(ret);
}
