use actix_web::{web, post, get, Responder, HttpResponse};
use sqlx::{query, postgres::PgPool};

use common::Register;

#[get("/test")]
async fn hello(pool: web::Data<PgPool>) -> impl Responder {
    let mut pool = pool.acquire().await.unwrap();

    let res: Vec<Register> = query!("select \"name\", username, email, \"password\" from \"user\";")
        .fetch_all(&mut pool)
        .await
        .unwrap()
        .into_iter()
        .map(|row| {
            Register {
                name: row.name,
                username: row.username,
                email: row.email,
                password: row.password
            }
        })
        .collect();


    HttpResponse::Ok().json(res)
}

#[post("/login")]
async fn login(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}





pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(login);
    cfg.service(hello);
}