use std::ops::{Deref, DerefMut};
use std::sync::Arc;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::rest_api::base::{BaseInfo, Ideantifier, NameType, ops::BaseOps};
use crate::rest_api::base::wrap::ActiveRecordWrap;
use crate::rest_api::json_models::issue::{IssueDto, IssueTagDto};
use crate::rest_api::json_models::issue::field::custom_field::{IssueCustomField, IssueStatus, StateIssueCustomField};
use crate::rest_api::json_models::issue::field::IssueStateType;
use crate::rest_api::json_models::issue::field::value::{FieldValue, StateBundleElement};
use crate::rest_api::service::issues::{fetch_issue_by_id, persist_changes};

pub type Issue = ActiveRecordWrap<IssueDto>;

#[async_trait]
pub trait IssueContract: BaseInfo + BaseOps + Sync {
    // async fn fields(&self) -> Vec<Box<IssueCustomField>> where Self: Sized;
    // async fn status(&self) -> Box<IssueCustomField> where Self: Sized;
    // async fn set_state_name(&mut self, status_name:String) -> () {
    //     self.
    //     match self.value {
    //         FieldValue::StateBundleElement {
    //             ref mut name,
    //             ..
    //         } => {
    //             *name = Some(status_name)
    //         },
    //         _ => ()
    //     };
    // }
    // async fn owner(&self) -> Vec<Box<dyn User>>;
}

// #[async_trait]
// impl IssueContract for Issue {
//     async fn fields(&self) -> Vec<Box<IssueCustomField>> {
//         unimplemented!()
//     }
//
//     async fn status(&self) -> Box<IssueStatus> {
//
//     }
// }

#[async_trait]
impl BaseInfo for Issue {
    async fn name(&self) -> NameType {
        unimplemented!()
    }

    async fn id(&self) -> Ideantifier {
        unimplemented!()
    }
}

#[async_trait]
impl BaseOps for Issue {
    async fn update(&mut self) -> &mut Self {
        fetch_issue_by_id(&self.http_client, self.origin.id.clone()).await
            .map(|new_origin| self.refresh(new_origin));
        self
    }

    async fn save(&mut self) -> &mut Self {
        let new_origin = persist_changes(&self.http_client, self.origin.clone(), self.inner.clone()).await;
        self.refresh(new_origin);
        self
    }
}

impl Issue {
    pub fn set_state_name(&mut self, status_type: IssueStateType) {
        let dto = &*self.inner;
        let new_fields = {
            let mut cloned_fields = dto.fields.clone();
            let (index, new_field) = cloned_fields.iter().enumerate()
                .filter_map(|(index, custom_field)| {
                    match custom_field.clone() {
                        IssueCustomField::StateIssueCustomField(state_custom_field) =>
                            match state_custom_field.value.clone() {
                                FieldValue::StateBundleElement(mut state_bundle_element) => {
                                    state_bundle_element.name = Some(status_type.clone());
                                    let new_field = IssueCustomField::StateIssueCustomField(
                                        StateIssueCustomField {
                                            value: FieldValue::StateBundleElement(state_bundle_element),
                                            ..state_custom_field
                                        }
                                    );
                                    Some((index, new_field))
                                }
                                _ => None
                            }
                        _ => None
                    }
                }).next()
                .unwrap();
            cloned_fields.remove(index);
            cloned_fields.push(new_field);
            cloned_fields
        };

        let new_mutable_state = IssueDto { fields: new_fields, ..dto.clone() };
        *Arc::make_mut(&mut self.inner) = new_mutable_state;
    }
    pub fn set_state(&mut self, state_type: IssueStateType) {
        self.set_state_name(state_type)
    }
}

pub mod search {
    #[derive(Clone)]
    pub enum IssueSearchParam {
        State(String),
        IssueId(String),
        ProjectName(String),
        TagTitle(String),
    }

    impl From<IssueSearchParam> for String {
        fn from(param: IssueSearchParam) -> Self {
            match param {
                // State: {Wait for merge}
                IssueSearchParam::State(state_name) => format!("State: {{{}}}", state_name),
                // issue id: PMS-2750
                IssueSearchParam::IssueId(issue_id) => format!("issue id: {{{}}}", issue_id),
                // project: {Paymash Server}
                IssueSearchParam::ProjectName(project_name) => format!("project: {{{}}}", project_name),
                // tag: Star
                IssueSearchParam::TagTitle(tag_title) => format!("tag: {}", tag_title)
            }
        }
    }
}