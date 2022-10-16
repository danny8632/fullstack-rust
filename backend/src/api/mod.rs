use actix_web::{web};

mod user;
mod auth;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .configure(user::config)
            .configure(auth::config)
    );
}