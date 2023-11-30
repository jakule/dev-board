use crate::middleware::{cors::cors_middleware, jwt::jwt_middleware};
use salvo::{
    prelude::{CatchPanic, Logger, OpenApi, SwaggerUi},
    Router,
};

use self::user::{delete_user, get_users, post_add_user, post_login, put_update_user};

mod pr;
mod static_routers;
pub mod user;

pub fn router() -> Router {
    let mut no_auth_routers = vec![Router::with_path("/api/login").post(post_login)];

    let _cors_handler = cors_middleware();

    let mut need_auth_routers = vec![Router::with_path("/api/users")
        .get(get_users)
        .post(post_add_user)
        .push(
            Router::with_path("<id>")
                .put(put_update_user)
                .delete(delete_user),
        )];

    let mut pr_routers = vec![Router::with_path("/api/prs").get(pr::get_prs)];

    let router = Router::new()
        //.hoop(_cors_handler)
        .hoop(Logger::new())
        .hoop(CatchPanic::new())
        .append(&mut no_auth_routers)
        .append(&mut pr_routers)
        .append(&mut static_routers::create_static_routers())
        .push(
            Router::new()
                .append(&mut need_auth_routers)
                .hoop(jwt_middleware()),
        );
    let doc = OpenApi::new("salvo web api", "0.0.1").merge_router(&router);
    router
        .push(doc.into_router("/api-doc/openapi.json"))
        .push(SwaggerUi::new("/api-doc/openapi.json").into_router("swagger-ui"))
}
