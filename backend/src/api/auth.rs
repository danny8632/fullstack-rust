use actix_web::{web, post, Responder, HttpResponse};
use sqlx::{query, postgres::PgPool};
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};

use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use sha2::Sha256;
use std::collections::BTreeMap;

use common::{Register, Login, User};

#[post("/login")]
async fn login(login: web::Json<Login>, pool: web::Data<PgPool>) -> impl Responder {
    let mut pool = pool.acquire().await.unwrap();

    let res = query!(
            "select * from \"user\" WHERE \"username\" = $1;",
            login.clone().username
        )
        .fetch_one(&mut pool)
        .await
        .unwrap();

    let user = User {
        id: res.id,
        name: res.name,
        username: res.username,
        email: res.email,
        created_on: res.created_on,
        last_login: res.last_login
    };

    let parsed_hash = PasswordHash::new(res.password.as_str()).unwrap();
    if !Argon2::default().verify_password(login.clone().password.as_bytes(), &parsed_hash).is_ok() {
        return HttpResponse::Ok().json("Wrong login creds")
    }

    // This means that were are now successfully logged in
    // So now we are going to create the key and the jwt
    let key: Hmac<Sha256> = Hmac::new_from_slice(b"some-secret").unwrap();
    let mut claims = BTreeMap::new();
    claims.insert("user", user);

    let token_str = claims.sign_with_key(&key).unwrap();

    return HttpResponse::Ok().body(token_str)
}

#[post("/register")]
async fn register(register: web::Json<Register>, pool: web::Data<PgPool>) -> impl Responder {
    let mut pool = pool.acquire().await.unwrap();

    // Checks to see if user already exists
    let query_result = query!(
            "SELECT id from \"user\" WHERE username = $1 OR email = $2",
            register.clone().username,
            register.clone().email
        )
        .fetch_all(&mut pool)
        .await
        .unwrap()
        .len();

    if query_result > 0 {
        return HttpResponse::Ok().json("Username or Email already in use.");
    }


    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password(register.clone().password.as_bytes(), &salt)
        .unwrap();

    let insert_query = query!(
        "INSERT INTO \"user\" (\"name\", \"username\", \"email\", \"password\") VALUES ($1, $2, $3, $4)",
        register.clone().name,
        register.clone().username,
        register.clone().email,
        password_hash.to_string()
    )
    .execute(&mut pool)
    .await
    .unwrap();


    HttpResponse::Ok().json(insert_query.rows_affected() > 0)
}


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(login);
    cfg.service(register);
}