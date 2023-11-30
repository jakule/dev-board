use salvo::{endpoint, Response};
use crate::app_response::{ErrRes, Res};
use crate::services::pr;

#[endpoint(tags("prs"))]
pub async fn get_prs(res: &mut Response) {
    let result = pr::prs().await;
    match result {
        Ok(data) => Res::with_data(data).into_response(res),
        Err(e) => ErrRes::with_err(&e.to_string()).into_response(res),
    }
}