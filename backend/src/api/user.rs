use actix_web::{web, get, Responder, HttpResponse};
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


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(hello);
}