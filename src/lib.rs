use std::net::TcpListener;
use std::sync::atomic::{AtomicU32, Ordering};

use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer};

use crate::models::User;

mod models;

pub fn get_users(amt: u32) -> Vec<User> {
    let mut users: Vec<User> = Vec::with_capacity(amt as usize);
    for index in 1..=amt {
        users.push(User {
            id: index,
            age: 25,
            first_name: format!("First_Name{}", index),
            last_name: format!("Last_Name{}", index),
            framework: "Rust (actix-web)".to_owned(),
        })
    }
    users
}

pub async fn get_users_async(amt: u32) -> Vec<User> {
    /*actix_web::rt::task::spawn_blocking(move || {*/
        /*get_users(amt)*/
    /*}).await.unwrap()*/
    get_users(amt)
}

static DO_MANY: AtomicU32 = AtomicU32::new(0);
const AMT: u32 = 1_000_000;

async fn users_many() -> HttpResponse {
    let dm = DO_MANY.fetch_add(1, Ordering::SeqCst);
    let users = if dm % 50 == 0 {
        web::block(|| get_users(AMT)).await.unwrap()
        //get_users(AMT)
        //get_users_async(AMT).await
    } else {
        let amt = 10;
        get_users(amt)
    };

    HttpResponse::Ok().json(users)
}

async fn users_few() -> HttpResponse {
    let users = get_users(2);

    HttpResponse::Ok().json(users)
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(move || 
        App::new()
            .route("users_many", web::get().to(users_many))
            .route("users_few", web::get().to(users_few)))
        .workers(num_cpus::get())
        .listen(listener)?
        .run();
    Ok(server)
}
