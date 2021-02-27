use async_trait::async_trait;
use crate::rest_api::json_models::issue::IssueTagDto;
use crate::rest_api::base::wrap::ActiveRecordWrap;
use crate::rest_api::base::{BaseInfo, NameType, Ideantifier};
use crate::rest_api::base::ops::BaseOps;
use crate::rest_api::service::tags::{persist_changes,fetch::fetch_tag_by_name};
use crate::rest_api::json_models::issue::field::FieldColor;
use crate::rest_api::error::Error;

pub type IssueTag = ActiveRecordWrap<IssueTagDto>;

// #[async_trait]
// impl BaseInfo for IssueTag {
//     async fn name(&self) -> NameType {
//         unimplemented!()
//     }
//
//     async fn id(&self) -> Ideantifier {
//         unimplemented!()
//     }
// }

#[async_trait]
impl BaseOps for IssueTag {
    async fn update(&mut self) -> &mut Self {
        fetch_tag_by_name(&self.http_client,  self.project_id.clone(),self.origin.name.clone()).await
            .map(|new_origin|self.refresh(new_origin));
        self
    }

    async fn save(&mut self) -> &mut Self {
        match persist_changes(&self.http_client, self.origin.clone(), self.inner.clone()).await {
            Ok(new_origin) => {
                self.refresh(new_origin);
            },
            Err(e) => log::warn!("Can't persist tag's changes, error: {error}", error = e)
        };
        self
    }
}
