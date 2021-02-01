use serde::{Serialize, Deserialize};

pub type Identifier = u32;
pub type VisibilityLevel = u8;
pub type AvatarUrl = Option<String>;
pub type MilestoneId = Option<Identifier>;

#[derive(Deserialize, Debug, Clone, PartialEq, Serialize)]
pub struct LastCommit {
    pub id: String,
    pub message: String,
    pub timestamp: String,
    pub url: String,
    pub author: Author,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Serialize)]
pub struct Author {
    pub name: String,
    pub email: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Serialize)]
pub struct Repository {
    pub name: String,
    pub url: String,
    pub description: String,
    pub homepage: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Serialize)]
pub struct Project {
    pub id: Option<Identifier>,
    pub name: String,
    pub description: String,
    pub web_url: String,
    pub avatar_url: AvatarUrl,
    pub git_ssh_url: String,
    pub git_http_url: String,
    pub namespace: String,
    pub visibility_level: VisibilityLevel,
    pub path_with_namespace: String,
    pub default_branch: String,
    pub homepage: String,
    pub url: String,
    pub ssh_url: String,
    pub http_url: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Serialize)]
pub struct User {
    pub name: String,
    pub username: String,
    pub avatar_url: Option<String>,
}

