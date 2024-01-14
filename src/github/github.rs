use ::reqwest::Client;
use anyhow::Error;
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

pub async fn post_graphql2<Q: GraphQLQuery, U: reqwest::IntoUrl>(
    client: &reqwest::Client,
    url: U,
    variables: Q::Variables,
) -> Result<graphql_client::Response<Q::ResponseData>, anyhow::Error> {
    let body = Q::build_query(variables);
    let reqwest_response = client.post(url).json(&body).send().await?;

    let code = &reqwest_response.status();
    if code != &200 {
        let text = reqwest_response.text().await?;
        return Err(anyhow::anyhow!("code: {}, body: {}", code, text));
    }

    reqwest_response.json().await.map_err(|e| e.into())
}

pub async fn fetch_pull_requests(
    token: String,
    cursor: Option<String>,
) -> Result<
    (
        Response<<Issues as GraphQLQuery>::ResponseData>,
        Option<String>,
    ),
    anyhow::Error,
> {
    let variables = issues::Variables {
        cursor: cursor.clone(),
        owner: "gravitational".to_string(),
        name: "teleport".to_string(),
    };

    let client = get_http_client(&token)?;
    let response_body =
        post_graphql2::<Issues, _>(&client, "https://api.github.com/graphql", variables).await?;

    let next_cursor = response_body
        .data
        .as_ref()
        .ok_or(anyhow::anyhow!("No data in response"))?
        .repository
        .as_ref()
        .ok_or(anyhow::anyhow!("No data in response/repository"))?
        .pull_requests
        .page_info
        .end_cursor
        .clone();

    Ok((response_body, next_cursor))
}

fn get_http_client(token: &String) -> Result<Client, Error> {
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
    Ok(client)
}

pub async fn fetch_pull_request_by_id(
    token: String,
    id: i64,
) -> Result<Response<<IssueByID as GraphQLQuery>::ResponseData>, anyhow::Error> {
    let variables = issue_by_id::Variables {
        id,
        owner: "gravitational".to_string(),
        name: "teleport".to_string(),
    };

    let client = get_http_client(&token)?;
    let response_body =
        post_graphql::<IssueByID, _>(&client, "https://api.github.com/graphql", variables).await?;

    Ok(response_body)
}
