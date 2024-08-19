use crate::imports::*;

#[derive(Deserialize, sqlx::FromRow, Serialize)]
pub struct PostStruct {
    pub id: String,
    pub product: String,
    pub price: i16,
    pub description: String,
}
