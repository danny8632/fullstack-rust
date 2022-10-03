use actix_web::{get, App, HttpResponse, HttpServer, Responder, middleware};
use common::TestStruct;


#[get("/test")]
async fn test() -> impl Responder {
    let test = TestStruct {
        name: String::from("hej")
    };

    HttpResponse::Ok().json(test)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let actix_port: u16 = std::env::var("ACTIX_PORT")
        .unwrap_or(String::from("8081"))
        .parse::<u16>()
        .unwrap();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::DefaultHeaders::new().add(("Access-Control-Allow-Origin", "http://localhost:8082")))
            .service(test)
    })
    .bind(("127.0.0.1", actix_port))?
    .run()
    .await
}