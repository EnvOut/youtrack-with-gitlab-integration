use youtrack_tools::rest_api::client::{YoutrackClientImpl, YoutrackClient};
use std::sync::Arc;
use gitlab_tools::models::hooks::GitlabHookRequest;
use youtrack_tools::rest_api::json_models::issue::field::IssueStateType;
use youtrack_tools::rest_api::json_models::issue::field::value::{FieldValue, StateBundleElement};
use youtrack_tools::rest_api::base::ops::BaseOps;
use youtrack_tools::rest_api::json_models::issue::{IssueDto, IssueTagDto};
use youtrack_tools::rest_api::base::wrap::ActiveRecordWrap;
use youtrack_tools::rest_api::json_models::issue::field::custom_field::StateIssueCustomField;
use youtrack_tools::rest_api::error::{Result as Res, Error};
use youtrack_tools::rest_api::error::Error as YoutrackError;
use actix_web::dev::Service;
use crate::service::definitions::base::{Filters, UpdateDef};
use youtrack_tools::rest_api::issue::Issue;
use youtrack_tools::rest_api::issue::search::IssueSearchParam;

pub struct YoutrackService {
    client: Arc<YoutrackClientImpl>
}

impl YoutrackService {
    pub fn new(client: YoutrackClientImpl) -> YoutrackService {
        let client = Arc::new(client);
        YoutrackService { client }
    }

    pub async fn get_client(&self) -> Arc<YoutrackClientImpl> {
        self.client.clone()
    }

    pub async fn update_status(&mut self, task_id: String) {
        match self.client.issue(task_id).await {
            Ok(mut issue) => {
                if let Some(state) = issue.get_state() {
                    if let FieldValue::StateBundleElement(StateBundleElement { name: Some(IssueStateType::WaitForMerge), .. }) = state.value {
                        issue.set_state(IssueStateType::Fixed);
                    }
                }
                issue.save().await;
            }
            Err(e) => {
                log::error!("{}", e)
            }
        }
    }

    /// Example:
    /// let project_id = settings::get_str("youtrack.project_id").unwrap();
    /// service.add_tag(project_id, task_id, "hello friend!9999".to_string()).await;
    pub async fn add_configured_tag(&mut self, project_id: String, task_id: String, (tag_name, style): (String, String)) {
        let results = (
            self.client.issue(task_id.clone()).await,
            self.client.find_or_new_tag(project_id, tag_name.clone(), style.clone()).await
        );
        match results {
            (Ok(mut issue), Ok(mut tag)) => {
                let mut tags = if let Some(tags) = &issue.tags {
                    let mut tags = tags.clone();
                    let dto = tag.clone();
                    // let dto = inner_tag.clone();
                    tags.push(dto);
                    // tags.push(dto);
                    tags
                } else {
                    vec![tag.clone()]
                };
                issue.tags = Some(tags);
                issue.save().await;
                log::info!("Added tag (title: {title}, style: {style}) the task {task} ", title = tag_name, style = style, task = task_id)
            }
            (issue_res, tag_res) => {
                if let Err(e) = issue_res {
                    log::warn!(r###"Can't fetch issue: "{task_id}", error: {error}"###, task_id = task_id, error = e)
                }
                if let Err(e) = tag_res {
                    log::warn!(r###"Can't fetch tag: "{tag_name}", error: {error}"###, tag_name = tag_name, error = e)
                }
            }
        };
    }

    pub async fn find_issues(&self, search_params: Vec<IssueSearchParam>) -> Res<Vec<Box<Issue>>> {
        self.client.find_issues(search_params).await
    }
}
