use crate::app_response::AppResult;
use crate::db::DB;
use crate::dtos::pr::PrResponse;
use crate::entities::pr::Pr;

pub async fn prs() -> AppResult<Vec<PrResponse>> {
    let db = DB.get().ok_or(anyhow::anyhow!(""))?;
    let prs = sqlx::query_as!(
        Pr,
        r#"
            SELECT id, title FROM pr
            "#,
    )
    .fetch_all(db)
    .await?;
    let res = prs
        .into_iter()
        .map(|pr| PrResponse {
            id: pr.id,
            title: pr.title,
        })
        .collect::<Vec<_>>();
    Ok(res)
}

pub async fn add_pr(id: String, title: String, data: String) -> AppResult<()> {
    let db = DB.get().ok_or(anyhow::anyhow!(""))?;
    sqlx::query!(
        r#"
            INSERT INTO pr (id, title, body)
            VALUES ($1, $2, $3) ON CONFLICT (id) DO
            UPDATE SET title = $2, body = $3
            "#,
        id,
        title,
        data
    )
    .execute(db)
    .await?;
    Ok(())
}
