use crate::{config::config_manager, imports::*, utils::auth_struct};
use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use sha2::Sha256;
use std::collections::BTreeMap;

pub async fn login_handler(mut req: Request<()>) -> tide::Result {
    let mut response = Response::builder(200).build();
    let user: auth_struct::LoginData = req.body_json().await?;
    let mut pg_conn = req.sqlx_conn::<Postgres>().await;
    let config = config_manager::load_config().expect("Config Error.");

    let user_result = find_user(&mut pg_conn, &user.username).await;
    match user_result {
        Ok(user_db) => {
            if unix::verify(user.password, &user_db.password) {
                if let Some(token) = generate_token(&config, &user_db).await? {
                    response.set_body(json!({
                        "token": token,
                    }));

                    return Ok(response);
                }
            }
            response.set_status(401);
            return Ok(response);
        }
        Err(_) => {
            response.set_status(401);
            return Ok(response);
        }
    }
}

async fn find_user(
    pg_conn: &mut sqlx::PgConnection,
    username: &str,
) -> tide::Result<auth_struct::UserStruct> {
    let user =
        sqlx::query_as::<_, auth_struct::UserStruct>("SELECT * FROM users WHERE username = $1")
            .bind(username)
            .fetch_one(pg_conn.acquire().await?)
            .await?;
    Ok(user)
}

async fn generate_token(
    config: &config_manager::Config,
    user: &auth_struct::UserStruct,
) -> tide::Result<Option<String>> {
    let key: Hmac<Sha256> = Hmac::new_from_slice(config.tide_secret.as_bytes())?;

    let mut claims = BTreeMap::new();
    claims.insert("userId", &user.id);
    claims.insert("username", &user.username);

    let token = claims.sign_with_key(&key)?;
    Ok(Some(token))
}
