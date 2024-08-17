mod config;
mod imports;
mod register_routes;
mod routes;
mod utils;
use crate::imports::*;
use http_types::headers::HeaderValue;
use tide::security::{CorsMiddleware, Origin};

#[async_std::main]
async fn main() -> tide::Result<()> {
    tide::log::start();
    let config = config::config_manager::load_config().expect("Config Error.");

    let mut app = tide::new();

    let cors = CorsMiddleware::new()
        .allow_credentials(true)
        .allow_origin(Origin::from("*"))
        .allow_methods("GET, POST".parse::<HeaderValue>()?)
        .allow_headers("Content-Type, Authorization".parse::<HeaderValue>()?);
    app.with(cors);

    let connection_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        config.database.username,
        config.database.password,
        config.database.host,
        config.database.port,
        config.database.db_name
    );
    app.with(SQLxMiddleware::<Postgres>::new(&connection_url).await?);

    register_routes::register_routes(&mut app);

    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
