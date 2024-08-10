mod imports;
mod register_routes;
mod routes;
mod utils;
use http_types::headers::HeaderValue;
use tide::security::{CorsMiddleware, Origin};

#[tokio::main]
async fn main() -> tide::Result<()> {
    // tide::log::start();
    let mut app = tide::new();

    let cors = CorsMiddleware::new()
        .allow_methods("GET, POST".parse::<HeaderValue>().unwrap())
        .allow_origin(Origin::from("*"))
        .allow_credentials(false);
    app.with(cors);

    register_routes::register_routes(&mut app);

    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
