use actix_web::{web, App, HttpServer, middleware};
use sqlx::postgres::PgPoolOptions;



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let actix_port: u16 = std::env::var("ACTIX_PORT")
        .unwrap_or(String::from("8081"))
        .parse::<u16>()
        .unwrap();

    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or(String::from("postgres://api:api123456@localhost/my_api"))
        .parse::<String>()
        .unwrap();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url.as_str())
        .await
        .expect("sql open");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::DefaultHeaders::new().add(("Access-Control-Allow-Origin", "http://localhost:80")))
            // .service(test)
    })
    .bind(("127.0.0.1", actix_port))?
    .run()
    .await
}