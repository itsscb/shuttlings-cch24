use super::Quote;

#[tracing::instrument(skip(pool))]
pub async fn reset_db(pool: &sqlx::PgPool) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM quotes")
        .execute(pool)
        .await
        .map(|_| ())
}

#[tracing::instrument(skip(pool))]
pub async fn draft(pool: &sqlx::PgPool, author: &str, quote: &str) -> Result<Quote, sqlx::Error> {
    let quote = sqlx::query_as!(
        Quote,
        "INSERT INTO quotes (author, quote) VALUES ($1, $2) RETURNING *",
        author,
        quote
    )
    .fetch_one(pool)
    .await?;

    Ok(quote)
}

#[tracing::instrument(skip(pool))]
pub async fn undo(
    pool: &sqlx::PgPool,
    id: uuid::Uuid,
    author: &str,
    quote: &str,
) -> Result<Quote, sqlx::Error> {
    let quote = sqlx::query_as!(
        Quote,
        "UPDATE quotes SET author = $2, quote = $3, version = version + 1 WHERE id = $1 RETURNING *",
        id,
        author,
        quote
    )
    .fetch_one(pool)
    .await?;

    Ok(quote)
}

#[tracing::instrument(skip(pool))]
pub async fn get(pool: &sqlx::PgPool, id: uuid::Uuid) -> Result<Quote, sqlx::Error> {
    let quote = sqlx::query_as!(Quote, "SELECT * FROM quotes WHERE id = $1", id)
        .fetch_one(pool)
        .await?;

    Ok(quote)
}

#[tracing::instrument(skip(pool))]
pub async fn list(
    pool: &sqlx::PgPool,
    page: Option<u32>,
) -> Result<(Vec<Quote>, u32, Option<u32>), sqlx::Error> {
    let limit = 3i64;
    #[allow(clippy::cast_lossless)]
    let offset = page.map_or(0i64, |page| (limit * (page - 1) as i64));

    let quotes = sqlx::query_as!(
        Quote,
        "SELECT * FROM quotes ORDER BY created_at LIMIT $1 OFFSET $2",
        limit,
        offset
    )
    .fetch_all(pool)
    .await?;
    let quotes_count: i64 = sqlx::query_scalar!("SELECT COUNT(*) FROM quotes")
        .fetch_one(pool)
        .await?
        .expect("Failed to get count");
    let page = page.unwrap_or(1);

    #[allow(
        clippy::cast_lossless,
        clippy::cast_sign_loss,
        clippy::cast_possible_truncation
    )]
    let (page, next_page) = if quotes_count > (page * limit as u32).into() {
        (page, Some(page + 1))
    } else {
        (page, None)
    };

    Ok((quotes, page, next_page))
}

#[tracing::instrument(skip(pool))]
pub async fn remove(pool: &sqlx::PgPool, id: uuid::Uuid) -> Result<Quote, sqlx::Error> {
    let quote = get(pool, id).await?;
    sqlx::query_as!(Quote, "DELETE FROM quotes WHERE id = $1", id)
        .execute(pool)
        .await?;

    Ok(quote)
}
