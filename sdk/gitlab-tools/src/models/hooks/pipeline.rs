use serde::{Serialize, Deserialize};
use crate::models::hooks::base::{LastCommit, User, Identifier, VisibilityLevel};

#[derive(Deserialize, Debug, Clone, PartialEq, Serialize)]
pub struct PipelineObjectAttributes {
    pub id: Identifier,
    pub ref_field: Option<String>,
    pub tag: bool,
    pub sha: String,
    pub before_sha: String,
    pub source: String,
    pub status: String,
    pub stages: Vec<String>,
    pub created_at: String,
    pub finished_at: String,
    pub duration: u32,
    pub variables: Vec<Variable>,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Serialize)]
pub struct Variable {
    pub key: String,
    pub value: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Serialize)]
pub struct PipelineMergeRequest {
    pub id: Identifier,
    pub iid: Identifier,
    pub title: String,
    pub source_branch: String,
    pub source_project_id: Identifier,
    pub target_branch: String,
    pub target_project_id: Identifier,
    pub state: String,
    pub merge_status: String,
    pub url: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Serialize)]
pub struct PipelineProject {
    pub id: Identifier,
    pub name: String,
    pub description: String,
    pub web_url: String,
    pub avatar_url: Option<String>,
    pub git_ssh_url: String,
    pub git_http_url: String,
    pub namespace: String,
    pub visibility_level: VisibilityLevel,
    pub path_with_namespace: String,
    pub default_branch: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Serialize)]
pub struct Build {
    pub id: Identifier,
    pub stage: String,
    pub name: String,
    pub status: String,
    pub created_at: String,
    pub started_at: Option<String>,
    pub finished_at: Option<String>,
    pub when: String,
    pub manual: bool,
    pub allow_failure: bool,
    pub user: User,
    pub runner: Option<Runner>,
    pub artifacts_file: ArtifactsFile,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Serialize)]
pub struct Runner {
    pub id: Identifier,
    pub description: String,
    pub active: bool,
    pub is_shared: bool,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Serialize)]
pub struct ArtifactsFile {
    pub filename: Option<String>,
    pub size: Option<String>,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Serialize)]
pub struct PipelineHook {
    // pub object_kind: String,
    pub object_attributes: PipelineObjectAttributes,
    pub merge_request: PipelineMergeRequest,
    pub user: User,
    pub project: PipelineProject,
    pub commit: LastCommit,
    pub builds: Vec<Build>,
}

#[cfg(test)]
pub mod test {
    use tokio;
    use crate::models::hooks::GitlabHookRequest;
    use env_utils::from_root_file;
    // use gitlab::webhooks::WebHook;

    #[tokio::test]
    async fn parse_example_ok() {
        let data = from_root_file("sdk/gitlab-tools/rest/pipeline/pipeline.example.json").await;
        let result = serde_json::from_slice::<GitlabHookRequest>(&data).unwrap();
    }

    // #[tokio::test]
    // async fn parse_real_ok() {
    //     let data = from_root_file("sdk/gitlab-tools/rest/merge_request/merge_request.real.json").await;
    //     let result = serde_json::from_slice::<GitlabHookRequest>(&data).unwrap();
    // }
}