use super::handler;
use super::model::User;
// use crate::schema::users;
use crate::utils::db::DbPool;
// use crate::AppState;
use actix_web::{get, post, put, web, HttpResponse, Responder};
// use serde::{Deserialize, Serialize};

#[post("/login")]
pub async fn signin() -> impl Responder {
    // TODO:
    HttpResponse::Ok().body("users signin")
}

#[post("")]
pub async fn signup(
    pool: web::Data<DbPool>,
    form: web::Json<handler::UserReq>,
) -> Result<HttpResponse, HttpResponse> {
    // println!("[signup]----------{}", form.email);

    // return Ok(HttpResponse::Ok().body("OK"));
    // -----------
    let conn = pool.get().expect("couldn't get db connection from pool");
    let bio = "bio";
    let user = web::block(move || User::signup(&conn, &form.email, bio))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().json(e.to_string())
        })?;
    Ok(HttpResponse::Ok().json(user))
}

#[get("")]
pub async fn me() -> impl Responder {
    // TODO:
    HttpResponse::Ok().body("users me")
}

#[put("")]
pub async fn update() -> impl Responder {
    // TODO:
    HttpResponse::Ok().body("users update")
}
