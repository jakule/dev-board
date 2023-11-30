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
