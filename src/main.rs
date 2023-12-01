use crate::github::github::fetch_pull_requests;
use crate::db::init_db_conn;
use crate::middleware::handle_404::handle_404;
use crate::routers::router;
use config::{CERT_KEY, CFG};
use salvo::catcher::Catcher;
use salvo::conn::rustls::{Keycert, RustlsConfig};
use salvo::prelude::*;
use std::time::Instant;
use tokio::sync::oneshot;
use tokio::time::{interval, Duration};

mod app_error;
mod app_response;
mod config;
mod db;
mod dtos;
mod entities;
mod github;
mod middleware;
mod routers;
mod services;
mod utils;

async fn perform_action(mut rx: oneshot::Receiver<()>) {
    let mut interval = interval(Duration::from_secs(15));

    let github_api_token =
        std::env::var("GITHUB_API_TOKEN").expect("Missing GITHUB_API_TOKEN env var");

    loop {
        tokio::select! {
            _ = &mut rx => {
                println!("Shutting down...");
                break;
            }
            _ = interval.tick() => {
                println!("Performing action at {:?}", Instant::now());

                let data = fetch_pull_requests(&github_api_token, None).await;
                match data {
                    Ok(data) => println!("data: {:?}", data),
                    Err(e) => {
                        println!("error: {:?}", e);
                    }
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    //At the same time, logs are only output to the terminal or file
    init_log();
    init_db_conn().await;
    let (tx, rx) = oneshot::channel();
    let router = router();
    let service: Service = router.into();
    let service = service.catcher(Catcher::default().hoop(handle_404));
    println!("🌪️ {} is starting ", &CFG.server.name);
    println!("🔄 listen on {}", &CFG.server.address);

    match CFG.server.ssl {
        true => {
            println!(
                "📖 Swagger API Page: https://{}/swagger-ui",
                &CFG.server.address.replace("0.0.0.0", "127.0.0.1")
            );
            let config = RustlsConfig::new(
                Keycert::new()
                    .cert(CERT_KEY.cert.clone())
                    .key(CERT_KEY.key.clone()),
            );
            let acceptor = TcpListener::new(&CFG.server.address)
                .rustls(config)
                .bind()
                .await;
            let server = Server::new(acceptor).serve_with_graceful_shutdown(
                service,
                async {
                    rx.await.ok();
                },
                None,
            );
            tokio::task::spawn(server);
        }
        false => {
            println!(
                "📖 Swagger API Page: http://{}/swagger-ui",
                &CFG.server.address.replace("0.0.0.0", "127.0.0.1")
            );
            let acceptor = TcpListener::new(&CFG.server.address).bind().await;
            let server = Server::new(acceptor).serve_with_graceful_shutdown(
                service,
                async {
                    rx.await.ok();
                },
                None,
            );
            tokio::task::spawn(server);
        }
    };

    let (tx_background, rx_background) = oneshot::channel();
    // Start background tasks here
    tokio::spawn(perform_action(rx_background));

    // Wait for Ctrl-C
    tokio::signal::ctrl_c().await.unwrap();

    // Then, start the shutdown...
    let _ = tx_background.send(());
    let _ = tx.send(());
}

fn init_log() {
    let _guard = clia_tracing_config::build()
        .filter_level(&CFG.log.filter_level)
        .with_ansi(CFG.log.with_ansi)
        .to_stdout(CFG.log.to_stdout)
        .directory(&CFG.log.directory)
        .file_name(&CFG.log.file_name)
        .rolling(&CFG.log.rolling)
        .init();
    tracing::info!("log level: {}", &CFG.log.filter_level);
}
