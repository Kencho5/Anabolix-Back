use crate::{
    imports::*,
    utils::{auth_struct, generate_id::get_id},
};

pub async fn register_handler(mut req: Request<()>) -> tide::Result {
    let mut response = Response::builder(200).build();
    let user: auth_struct::RegisterData = req.body_json().await?;
    let user_id = get_id();
    let mut pg_conn = req.sqlx_conn::<Postgres>().await;

    if !register_user(&mut pg_conn, user_id, &user).await {
        response.set_status(500);
        return Ok(response);
    }
    return Ok(response);
}

async fn register_user(
    pg_conn: &mut sqlx::PgConnection,
    user_id: String,
    user: &auth_struct::RegisterData,
) -> bool {
    let pass_hash = bcrypt::hash(&user.password).unwrap();
    let registration_result =
        sqlx::query("INSERT INTO users(id, username, password) VALUES($1, $2, $3)")
            .bind(user_id.to_string())
            .bind(&user.username)
            .bind(&pass_hash)
            .execute(pg_conn)
            .await;

    registration_result.is_ok()
}
