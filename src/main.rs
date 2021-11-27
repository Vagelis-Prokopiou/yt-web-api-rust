#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let listener = std::net::TcpListener::bind("0.0.0.0:8082").expect("Failed to bind to port 80");
    api::run(listener)?.await
}
