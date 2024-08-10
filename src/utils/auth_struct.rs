use crate::imports::*;

#[derive(Deserialize)]
pub struct RegisterData {
    pub username: String,
    pub password: String,
}
