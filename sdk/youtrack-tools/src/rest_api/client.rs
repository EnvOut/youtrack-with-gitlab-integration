use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use hyper::client::HttpConnector;
use tokio::time::Duration;

use crate::rest_api::base::{BaseInfo, NameType};
use crate::rest_api::base::client::HttpClient;
use crate::rest_api::base::ops::BaseOps;
use crate::rest_api::error::Error as YoutrackError;
use crate::rest_api::error::Result as Res;
use crate::rest_api::issue::Issue;
use crate::rest_api::issue::search::IssueSearchParam;
use crate::rest_api::json_models::issue::IssueTagDto;
use crate::rest_api::service::issues::{fetch_issue_by_id, find_issues};
use crate::rest_api::service::tags::fetch::fetch_tag_by_name;
use crate::rest_api::tag::IssueTag;

pub struct YoutrackClientImpl {
    client: Arc<HttpClient>,
    config: Config,
}

#[derive(Clone, Debug)]
pub struct Config {
    pub host: String,
    pub token: String,
}

#[async_trait]
pub trait YoutrackClient: Sync {
    // async fn users(&self) -> Vec<Box<dyn User>>;
    // async fn user(&self, name: NameType) -> Vec<Box<dyn User>>;
    // async fn tasks(&self) -> Vec<Box<dyn Task>>;
    async fn issue(&self, name: NameType) -> Res<Box<Issue>>;
    async fn find_issues(&self, search_params: Vec<IssueSearchParam>) -> Res<Vec<Box<Issue>>>;
    async fn new_tag(&self, project_id: String, name: NameType, style: String) -> Res<Box<IssueTag>> ;
    async fn find_or_new_tag(&self, project_id: String, name: NameType, style: String) -> Res<Box<IssueTag>>;
    async fn tag(&self, project_id: String, name: NameType) -> Res<Box<IssueTag>>;
    // async fn projects(&self) -> Vec<Box<dyn Project>>;
    // async fn project(&self, name: NameType) -> Box<dyn Project>;
}

impl YoutrackClientImpl {
    pub async fn new(domain: String, bearer_token: String) -> Result<YoutrackClientImpl, ()> {
        let url = "something".parse::<hyper::Uri>().unwrap();
        let mut req = hyper::Request::new(url);
        req.headers_mut().insert(hyper::header::AUTHORIZATION, "Bearer perm:token".parse().unwrap());

        let config = Config { host: domain, token: bearer_token };
        let youtrack_client = YoutrackClientImpl {
            client: Arc::new(HttpClient::new(config.clone())),
            config,
        };
        Ok(youtrack_client)
    }
}

#[async_trait]
impl YoutrackClient for YoutrackClientImpl {
    async fn issue(&self, name: NameType) -> Res<Box<Issue>> {
        let http_client = Arc::new(HttpClient::new(self.config.clone()));
        let origin = fetch_issue_by_id(&http_client, name.clone()).await;
        origin.map(|origin_dto| {
            let project_id = origin_dto.project.id.clone();
            box Issue::new(http_client, origin_dto, project_id)
        })
    }

    async fn find_issues(&self, search_params: Vec<IssueSearchParam>) -> Res<Vec<Box<Issue>>> {
        let http_client = Arc::new(HttpClient::new(self.config.clone()));
        find_issues(&http_client, search_params).await
            .map(|results| {
                results.iter().cloned()
                    .map(|origin_dto| {
                        let project_id = origin_dto.project.id.clone();
                        box Issue::new(http_client.clone(), origin_dto, project_id)
                    })
                    .collect()
            })
    }

    async fn new_tag(&self, project_id: String, name: NameType, style: String) -> Res<Box<IssueTag>> {
        let http_client = Arc::new(HttpClient::new(self.config.clone()));
        let origin = IssueTagDto::new(name, style);
        let mut tag = IssueTag::new(http_client, origin, project_id);
        let new = tag.save().await;
        Ok(box tag)
    }

    async fn find_or_new_tag(&self, project_id: String, name: NameType, style: String) -> Res<Box<IssueTag>> {
        let old_tag = self.tag(project_id.clone(), name.clone()).await;
        let new_tag = self.new_tag(project_id, name, style).await;
        old_tag.or_else(|e| new_tag)
    }

    async fn tag(&self, project_id: String, name: NameType) -> Res<Box<IssueTag>> {
        let http_client = Arc::new(HttpClient::new(self.config.clone()));
        fetch_tag_by_name(&http_client, project_id.clone(), name.clone()).await
            .map(|origin| {
                let tag = IssueTag::new(http_client.clone(), origin, project_id);
                tag
            })
            .map(|it| box it)
    }
}