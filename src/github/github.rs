use ::reqwest::Client;
use graphql_client::{reqwest::post_graphql, GraphQLQuery, Response};

type URI = String;
type DateTime = String;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/github/schema.docs.graphql",
    query_path = "src/github/gh_issue.graphql",
    response_derives = "Debug,Serialize"
)]
pub struct Issues;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/github/schema.docs.graphql",
    query_path = "src/github/gh_issue_by_id.graphql",
    response_derives = "Debug,Serialize"
)]
pub struct IssueByID;

pub async fn fetch_pull_requests(
    token: String,
    cursor: Option<String>,
) -> Result<
    (
        Response<<Issues as GraphQLQuery>::ResponseData>,
        Option<String>,
    ),
    reqwest::Error,
> {
    let variables = issues::Variables {
        cursor: cursor.clone(),
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

    let next_cursor = response_body
        .data
        .as_ref()
        .unwrap()
        .repository
        .as_ref()
        .unwrap()
        .pull_requests
        .page_info
        .end_cursor
        .clone();

    // TODO:
    // next_cursor: Some("...")
    // prs: []
    // thread 'tokio-runtime-worker' panicked at src/github/github.rs:58:10:
    // called `Option::unwrap()` on a `None` value

    Ok((response_body, next_cursor))
}

pub async fn fetch_pull_request_by_id(
    token: String,
    id: i64,
) -> Result<Response<<IssueByID as GraphQLQuery>::ResponseData>, reqwest::Error> {
    let variables = issue_by_id::Variables {
        id: id,
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
        post_graphql::<IssueByID, _>(&client, "https://api.github.com/graphql", variables)
            .await
            .unwrap();

    Ok(response_body)
}
