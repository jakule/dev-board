use ::reqwest::Client;
use graphql_client::{reqwest::post_graphql, GraphQLQuery, Response};
use std::error::Error;

type URI = String;
type DateTime = String;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/github/schema.docs.graphql",
    query_path = "src/github/gh_issue.graphql",
    response_derives = "Debug"
)]
pub struct Issues;

pub async fn fetch_pull_requests(
    token: &String,
    cursor: Option<String>,
) -> Result<Response<<Issues as GraphQLQuery>::ResponseData>, Box<dyn Error>> {
    let variables = issues::Variables {
        cursor,
        owner: "gravitational".to_string(),
        name: "teleport".to_string(),
    };

    let client = Client::builder()
        .user_agent("graphql-rust/0.10.0")
        .default_headers(
            std::iter::once((
                reqwest::header::AUTHORIZATION,
                reqwest::header::HeaderValue::from_str(&format!("Bearer {}", token)).unwrap(),
            ))
            .collect(),
        )
        .build()?;

    let response_body =
        post_graphql::<Issues, _>(&client, "https://api.github.com/graphql", variables)
            .await
            .unwrap();

    println!("{:#?}", response_body);
    Ok(response_body)
}
