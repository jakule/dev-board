use crate::app_response::AppResult;
use crate::db::DB;
use crate::dtos::pr::PrResponse;
use crate::entities::pr::Pr;
use crate::InferenceResponse;

pub async fn prs() -> AppResult<Vec<PrResponse>> {
    let db = DB.get().ok_or(anyhow::anyhow!(""))?;
    let prs = sqlx::query_as!(
        Pr,
        r#"
            SELECT id, title, score, opened_at FROM prs
            ORDER BY prs.opened_at + interval '1' minute * FLOOR(prs.score * 60)
            LIMIT 10
            "#,
    )
    .fetch_all(db)
    .await?;
    let res = prs
        .into_iter()
        .map(|pr| PrResponse {
            id: pr.id,
            title: pr.title,
            score: pr.score,
            opened_at: chrono::DateTime::from_naive_utc_and_offset(pr.opened_at, chrono::Utc),
            should_close_at: chrono::DateTime::from_naive_utc_and_offset(
                pr.opened_at + chrono::Duration::seconds((pr.score * 60.0) as i64),
                chrono::Utc,
            ),
        })
        .collect::<Vec<_>>();
    Ok(res)
}

pub async fn add_pr(
    id: String,
    title: String,
    data: String,
    status: String,
    inference_data: InferenceResponse,
) -> AppResult<()> {
    let db = DB.get().ok_or(anyhow::anyhow!(""))?;

    sqlx::query!(
        r#"
            INSERT INTO prs (id, title, data, status, score, opened_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, NOW()) ON CONFLICT (id) DO
            UPDATE SET title = $2, data = $3, status = $4, score = $5, opened_at = $6, updated_at = NOW()
            "#,
        id,
        title,
        data,
        status,
        inference_data.prediction,
        inference_data.created_at.naive_utc(),
    )
    .execute(db)
    .await?;
    Ok(())
}

pub async fn get_not_updated() -> AppResult<Vec<Pr>> {
    let db = DB.get().ok_or(anyhow::anyhow!(""))?;

    let res = sqlx::query_as!(
        Pr,
        r#"
            SELECT id, title, score, opened_at FROM prs
            WHERE status = 'open' AND updated_at < (select min(updated_at) from prs where status = 'open')
            "#,
    )
    .fetch_all(db)
    .await?;
    Ok(res)
}

pub async fn update_sync_metadata(
    owner: String,
    repo: String,
    last_cursor: Option<String>,
) -> AppResult<()> {
    let db = DB.get().ok_or(anyhow::anyhow!(""))?;

    sqlx::query!(
        r#"
            INSERT INTO pr_sync_metadata (owner, repo, last_cursor)
            VALUES ($1, $2, $3) ON CONFLICT (owner, repo) DO
            UPDATE SET last_cursor = $3
            "#,
        owner,
        repo,
        last_cursor,
    )
    .execute(db)
    .await?;
    Ok(())
}

pub async fn get_sync_metadata(owner: String, repo: String) -> AppResult<Option<String>> {
    let db = DB.get().ok_or(anyhow::anyhow!(""))?;

    let res = sqlx::query!(
        r#"
            SELECT last_cursor FROM pr_sync_metadata
            WHERE owner = $1 AND repo = $2
            "#,
        owner,
        repo,
    )
    .fetch_optional(db)
    .await?;
    Ok(res.map(|r| r.last_cursor).flatten())
}
