use crate::imports::*;

pub async fn login_handler(req: Request<()>) -> tide::Result {
    let response = Response::builder(200).build();

    return Ok(response);
}
