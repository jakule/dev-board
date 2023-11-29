use rust_embed::RustEmbed;
use salvo::{
    endpoint, http::ResBody, hyper::body::Bytes, serve_static::static_embed, Response, Router,
};

#[derive(RustEmbed)]
#[folder = "web/build"]
struct Assets;

pub fn create_static_routers() -> Vec<Router> {
    let static_router =
        Router::with_path("<**path>").get(static_embed::<Assets>().fallback("index.html"));
    let icon_router = Router::with_path("favicon.ico").get(get_icon);
    vec![static_router, icon_router]
}

#[endpoint(tags("comm"))]
pub async fn get_icon(res: &mut Response) {
    let icon = Assets::get("favicon.ico").unwrap();
    res.body(ResBody::Once(Bytes::from(icon.data.to_vec())));
}
