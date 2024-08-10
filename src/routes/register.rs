use crate::{imports::*, utils::auth_struct};

pub async fn register_handler(mut req: Request<()>) -> tide::Result {
    let mut response = Response::builder(200).build();
    let user: auth_struct::RegisterData = req.body_json().await?;

    response.set_body(json!({
        "username": user.username
    }));
    return Ok(response);
}
