use crate::imports::*;
use crate::routes::*;

pub fn register_routes(app: &mut Server<AppState>) {
    app.at("/api/location")
        .with(GovernorMiddleware::per_hour(600).unwrap())
        .post(location::location_handler);

    // AUTH ROUTES
    app.at("/api/login")
        .with(GovernorMiddleware::per_hour(600).unwrap())
        .post(login::login_handler);

    app.at("/api/register")
        .with(GovernorMiddleware::per_hour(600).unwrap())
        .post(register::register_handler);

    app.at("/api/posts")
        .with(GovernorMiddleware::per_hour(600).unwrap())
        .post(posts::posts_handler);
}
