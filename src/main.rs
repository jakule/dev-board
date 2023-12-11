use crate::db::init_db_conn;
use crate::github::github::fetch_pull_requests;
use crate::github::github::issues::PullRequestState;
use crate::github::github::Issues;
use crate::middleware::handle_404::handle_404;
use crate::routers::router;
use crate::services::pr::update_sync_metadata;
use crate::services::pr::{add_pr, get_sync_metadata};
use chrono::{DateTime, Utc};
use config::{CERT_KEY, CFG};
use futures::StreamExt;
use graphql_client::GraphQLQuery;
use reqwest::Client;
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

async fn process_data(data: <Issues as GraphQLQuery>::ResponseData) {
    let prs = data.repository.unwrap().pull_requests.edges.unwrap();

    let futures = futures::stream::iter(prs).for_each_concurrent(None, |edge| async move {
        let data = serde_json::to_string(&edge).unwrap();
        // println!("edge: {}", data);

        let pr = edge.as_ref().unwrap().node.as_ref().unwrap();

        let state_string = match pr.state {
            PullRequestState::OPEN => "OPEN".to_string(),
            PullRequestState::CLOSED => "CLOSED".to_string(),
            PullRequestState::MERGED => "MERGED".to_string(),
            _ => "UNKNOWN".to_string(),
        };

        let inf_data = serde_json::to_string(&pr).unwrap();
        let inference_resp = fetch_expected_end_date(inf_data.clone().into()).await;
        match inference_resp {
            Ok(ref resp) => {
                println!("inference_resp: {:?}", resp);
            }
            Err(e) => {
                println!("error: {:?}", e);
                return;
            }
        }

        let res = add_pr(
            pr.number.to_string(),
            pr.title.clone(),
            data,
            state_string,
            inference_resp.unwrap(),
        )
        .await;
        match res {
            Ok(_) => {
                println!("Successfully added PR");
            }
            Err(e) => {
                println!("error: {:?}", e);
            }
        }
    });

    futures.await;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct InferenceResponse {
    prediction: f64,
    created_at: DateTime<Utc>,
}

async fn fetch_expected_end_date(
    pull_request_state: reqwest::Body,
) -> Result<InferenceResponse, anyhow::Error> {
    let client = Client::new();
    let response = client
        .post("http://localhost:8000/api/pr")
        .body(pull_request_state)
        .send()
        .await?;

    match response.status() {
        StatusCode::OK => {
            let r = response.json::<InferenceResponse>().await?;
            // println!("response: {:?}", r);
            Ok(r)
        }
        _ => {
            println!("Received response status: {:?}", &response.status());
            let r = response.text().await?;
            Err(anyhow::anyhow!(r))
        }
    }
}

use anyhow::Result as AnyhowResult;
use serde_derive::{Deserialize, Serialize};

async fn perform_action(mut rx: oneshot::Receiver<()>) -> AnyhowResult<()> {
    let mut interval = interval(Duration::from_secs(15 * 60));

    let github_api_token =
        std::env::var("GITHUB_API_TOKEN").expect("Missing GITHUB_API_TOKEN env var");

    loop {
        tokio::select! {
            _ = &mut rx => {
                println!("Shutting down...");
                break Ok(());
            }
            _ = interval.tick() => {
                println!("Performing action at {:?}", Instant::now());

                let cursor_db = get_sync_metadata("gravitational".to_string(), "teleport".to_string()).await?;

                let mut cursor: Option<String> = cursor_db;
                   println!("cursor: {:?}", cursor);

                loop {
                    let x = fetch_pull_requests(github_api_token.clone(), cursor.clone()).await;
                    match x {
                        Ok((response, next_cursor)) => {
                            match response.data {
                                Some(data) => {
                                    process_data(data).await;
                                },
                                None => println!("No data found in response"),
                            }

                            if next_cursor.is_none() {
                                break;
                            }

                            println!("next_cursor: {:?}", next_cursor);

                            cursor = next_cursor;

                            let result = update_sync_metadata(
                                "gravitational".to_string(),
                                "teleport".to_string(),
                                cursor.clone(),
                            ).await;

                            if let Err(e) = result {
                                println!("error: {:?}", e);
                                break;
                            }

                            tokio::time::sleep(Duration::from_secs(10)).await
                        },
                        Err(e) => {
                            println!("error: {:?}", e);
                        }
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
