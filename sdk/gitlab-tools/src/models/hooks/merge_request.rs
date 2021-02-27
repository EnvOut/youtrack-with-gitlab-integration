use serde::{self, Deserialize, Serialize};
use crate::models::hooks::base::{LastCommit, Project, Repository, User, Identifier, MilestoneId, AvatarUrl};
use crate::models::hooks::merge_request::custom::{MergeRequestState, MergeRequestAction};

#[derive(Deserialize, Debug, Clone, PartialEq, Serialize)]
pub struct MRObjectAttributes {
    pub id: Identifier,
    pub target_branch: String,
    pub source_branch: String,
    pub source_project_id: Identifier,
    pub author_id: Identifier,
    pub assignee_id: Option<Identifier>,
    pub title: String,
    pub created_at: String,
    pub updated_at: String,
    pub milestone_id: MilestoneId,
    pub state: MergeRequestState,
    pub merge_status: String,
    pub target_project_id: Identifier,
    pub iid: Identifier,
    pub description: String,
    pub source: Project,
    pub target: Project,
    pub last_commit: LastCommit,
    pub work_in_progress: bool,
    pub url: String,
    pub action: Option<MergeRequestAction>,
    // pub action: Option<String>,
    pub assignee: Option<User>,
    pub merge_when_pipeline_succeeds: Option<bool>,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Serialize)]
pub struct Label {
    pub id: Identifier,
    pub title: String,
    pub color: String,
    pub project_id: Identifier,
    pub created_at: String,
    pub updated_at: String,
    pub template: bool,
    pub description: String,
    pub group_id: Identifier,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Serialize)]
pub struct Changes {
    pub updated_by_id: Option<UpdatedById>,
    pub updated_at: Option<UpdatedAt>,
    pub labels: Option<Labels>,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Serialize)]
pub struct UpdatedById {
    pub previous: Option<Identifier>,
    pub current: Identifier,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Serialize)]
pub struct UpdatedAt {
    pub previous: Option<String>,
    pub current: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Serialize)]
pub struct Labels {
    pub previous: Vec<Label>,
    pub current: Vec<Label>,
}

pub mod custom {
    use serde::{Deserialize, Deserializer, Serialize};
    use serde_json::Value;
    use serde::de::{Error, Unexpected};
    use std::any::Any;

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum MergeRequestState {
        Opened,
        Closed,
        Locked,
        Merged,
        Other(String)
    }

    impl<'de> Deserialize<'de> for MergeRequestState {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de>, {
            let value = <Value as Deserialize>::deserialize(deserializer)?;

            let result = value.as_str().ok_or({
                D::Error::invalid_value(
                    Unexpected::Seq, &format!("Wrong action type: {:?}", &value).as_str())
            }).map(|value| match value {
                "opened" => MergeRequestState::Opened,
                "closed" => MergeRequestState::Closed,
                "locked" => MergeRequestState::Locked,
                "merged" => MergeRequestState::Merged,
                // "merge" => MergeRequestAction::Merge,
                other => MergeRequestState::Other(other.to_string())
            })
                .map_err(|err| {
                    D::Error::invalid_value(
                        Unexpected::Other("web hook"),
                        &format!("{:?}", err).as_str(),
                    )
                });
            result
        }
    }


    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub enum MergeRequestAction {
        Merge,
        Other(String),
    }

    impl<'de> Deserialize<'de> for MergeRequestAction {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de>, {
            let value = <Value as Deserialize>::deserialize(deserializer)?;

            let result = value.as_str().ok_or({
                D::Error::invalid_value(
                    Unexpected::Seq, &format!("Wrong action type: {:?}", &value).as_str())
            }).map(|value| match value {
                "merge" => MergeRequestAction::Merge,
                other => MergeRequestAction::Other(other.to_string()),
            })
                .map_err(|err| {
                    D::Error::invalid_value(
                        Unexpected::Other("web hook"),
                        &format!("{:?}", err).as_str(),
                    )
                });
            result
        }
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq, Serialize)]
pub struct MergeRequestHook {
    pub user: User,
    pub project: Project,
    pub repository: Repository,
    pub object_attributes: MRObjectAttributes,
    pub labels: Vec<Label>,
    pub changes: Option<Changes>,
}

#[cfg(test)]
pub mod test {
    use tokio;
    use crate::models::hooks::GitlabHookRequest;
    use env_utils::from_root_file;
    // use gitlab::webhooks::WebHook;

    #[tokio::test]
    async fn parse_example_ok() {
        let data = from_root_file("sdk/gitlab-tools/rest/merge_request/merge_request.example.json").await;
        let result = serde_json::from_slice::<GitlabHookRequest>(&data).unwrap();
    }

    #[tokio::test]
    async fn parse_real_ok() {
        let data = from_root_file("sdk/gitlab-tools/rest/merge_request/merge_request.real.json").await;
        let result = serde_json::from_slice::<GitlabHookRequest>(&data).unwrap();
    }
}
