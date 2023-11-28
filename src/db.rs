use crate::config::CFG;
use sqlx::PgPool;
use tokio::sync::OnceCell;
pub static DB: OnceCell<PgPool> = OnceCell::const_new();
pub async fn init_db_conn() {
    DB.get_or_init(|| async {
        PgPool::connect(&CFG.database.database_url)
            .await
            .expect("")
    })
    .await;
}
