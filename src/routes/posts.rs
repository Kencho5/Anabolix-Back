use crate::{imports::*, utils::post_struct};

pub async fn posts_handler(req: Request<()>) -> tide::Result {
    let posts = get_posts(&req).await?;
    let json = serde_json::to_string(&posts)?;

    let response = Response::builder(200)
        .body(json)
        .content_type("application/json")
        .build();

    Ok(response)
}

async fn get_posts(req: &Request<()>) -> tide::Result<Vec<post_struct::PostStruct>> {
    let query = "SELECT * FROM posts ORDER BY time_posted DESC";

    let mut pg_conn = req.sqlx_conn::<Postgres>().await;
    let posts = sqlx::query_as::<_, post_struct::PostStruct>(&query)
        .fetch_all(pg_conn.acquire().await?)
        .await?;
    Ok(posts)
}
