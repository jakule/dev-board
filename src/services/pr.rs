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

pub async fn add_pr(id: String, title: String, data: String, status: String) -> AppResult<()> {
    let db = DB.get().ok_or(anyhow::anyhow!(""))?;
    sqlx::query!(
        r#"
            INSERT INTO pr (id, title, data, status)
            VALUES ($1, $2, $3, $4) ON CONFLICT (id) DO
            UPDATE SET title = $2, data = $3, status = $4
            "#,
        id,
        title,
        data,
        status
    )
    .execute(db)
    .await?;
    Ok(())
}
