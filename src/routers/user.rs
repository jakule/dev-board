use crate::{
    app_response::AppResult,
    app_response::{ErrRes, Res},
    dtos::user::{UserAddRequest, UserLoginRequest, UserLoginResponse, UserUpdateRequest},
    services::user,
};
use salvo::{
    endpoint,
    http::cookie::Cookie,
    oapi::extract::{JsonBody, PathParam},
    Request, Response,
};

#[endpoint(tags("comm"))]
pub async fn post_login(req: JsonBody<UserLoginRequest>, res: &mut Response) {
    let result: AppResult<UserLoginResponse> = user::login(req.0).await;
    match result {
        Ok(data) => {
            let jwt_token = data.token.clone();
            let cookie = Cookie::build(("jwt_token", jwt_token))
                .path("/")
                .http_only(true)
                .build();
            res.add_cookie(cookie);
        }
        Err(e) => ErrRes::with_err(&e.to_string()).into_response(res),
    }
}

#[endpoint(tags("users"))]
pub async fn post_add_user(req: JsonBody<UserAddRequest>, res: &mut Response) {
    let result = user::add_user(req.0).await;
    match result {
        Ok(data) => Res::with_data(data).into_response(res),
        Err(e) => ErrRes::with_err(&e.to_string()).into_response(res),
    }
}

#[endpoint(  tags("users"),
parameters(
    ("id", description = "user id"),
))]
pub async fn put_update_user(req: &mut Request, res: &mut Response) {
    let req: UserUpdateRequest = req.extract().await.unwrap();
    let result = user::update_user(req).await;
    match result {
        Ok(data) => Res::with_data(data).into_response(res),
        Err(e) => ErrRes::with_err(&e.to_string()).into_response(res),
    }
}

#[endpoint(tags("users"))]
pub async fn delete_user(id: PathParam<String>, res: &mut Response) {
    let result = user::delete_user(id.0).await;
    match result {
        Ok(_) => Res::with_data(()).into_response(res),
        Err(e) => ErrRes::with_err(&e.to_string()).into_response(res),
    }
}

#[endpoint(tags("users"))]
pub async fn get_users(res: &mut Response) {
    let result = user::users().await;
    match result {
        Ok(data) => Res::with_data(data).into_response(res),
        Err(e) => ErrRes::with_err(&e.to_string()).into_response(res),
    }
}
