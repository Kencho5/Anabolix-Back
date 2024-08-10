use crate::imports::*;
use crate::routes::*;

pub fn register_routes(app: &mut Server<()>) {
    // AUTH ROUTES
    app.at("/login")
        .with(GovernorMiddleware::per_hour(600).unwrap())
        .post(login::login_handler);

    app.at("/register")
        .with(GovernorMiddleware::per_hour(600).unwrap())
        .post(register::register_handler);
}
