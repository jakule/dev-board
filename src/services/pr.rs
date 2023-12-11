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
            ORDER BY score DESC
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
            INSERT INTO prs (id, title, data, status, score, opened_at)
            VALUES ($1, $2, $3, $4, $5, $6) ON CONFLICT (id) DO
            UPDATE SET title = $2, data = $3, status = $4, score = $5, opened_at = $6
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
