use crate::imports::*;

#[derive(Deserialize)]
pub struct RegisterData {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginData {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, sqlx::FromRow)]
pub struct UserStruct {
    pub id: String,
    pub username: String,
    pub password: String,
}
