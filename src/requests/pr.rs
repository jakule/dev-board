use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Root {
    pub data: Data,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Data {
    pub repository: Repository,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Repository {
    #[serde(rename = "pullRequests")]
    pub pull_requests: PullRequests,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PullRequests {
    #[serde(rename = "pageInfo")]
    pub page_info: PageInfo,
    pub edges: Vec<Edge>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PageInfo {
    #[serde(rename = "hasNextPage")]
    pub has_next_page: bool,
    #[serde(rename = "endCursor")]
    pub end_cursor: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Edge {
    pub node: Node,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Node {
    pub title: String,
    pub url: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    #[serde(rename = "mergedAt")]
    pub merged_at: String,
    #[serde(rename = "bodyText")]
    pub body_text: String,
    pub number: i64,
    #[serde(rename = "changedFiles")]
    pub changed_files: i64,
    pub deletions: i64,
    pub additions: i64,
    #[serde(rename = "isDraft")]
    pub is_draft: bool,
    pub labels: Labels,
    #[serde(rename = "baseRef")]
    pub base_ref: BaseRef,
    pub state: String,
    pub author: Author,
    pub comments: Comments,
    #[serde(rename = "timelineItems")]
    pub timeline_items: TimelineItems,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Labels {
    pub edges: Vec<Edge2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Edge2 {
    pub node: Node2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Node2 {
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BaseRef {
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Author {
    pub login: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Comments {
    #[serde(rename = "totalCount")]
    pub total_count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimelineItems {
    pub edges: Vec<Edge3>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Edge3 {
    pub node: Node3,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Node3 {
    #[serde(rename = "__typename")]
    pub typename: String,
    pub commit: Option<Commit>,
    pub body: Option<String>,
    pub state: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<String>,
    pub comments: Option<Comments2>,
    pub author: Option<Author2>,
    #[serde(rename = "bodyText")]
    pub body_text: Option<String>,
    pub id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Commit {
    #[serde(rename = "committedDate")]
    pub committed_date: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Comments2 {
    pub edges: Vec<Edge4>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Edge4 {
    pub node: Node4,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Node4 {
    #[serde(rename = "createdAt")]
    pub created_at: String,
    pub body: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Author2 {
    pub login: String,
}
