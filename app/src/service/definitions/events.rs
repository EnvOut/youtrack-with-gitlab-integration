use config::{Value, ConfigError};
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use crate::settings;
use crate::service::definitions::operations::Operation;

#[derive(Default, Debug, Clone)]
pub struct OnPipelineEvent {
    started: Vec<Operation>,
    success: Vec<Operation>,
    failed: Vec<Operation>,
}

#[derive(Default, Debug, Clone)]
pub struct OnMergeRequestEvent {
    created: Vec<Operation>,
    updated: Vec<Operation>,
    conflict: Vec<Operation>,
    merged: Vec<Operation>,
}

pub enum GitlabEvent {
    OnComment(Vec<Operation>),
    OnPipeline(OnPipelineEvent),
    OnMergeRequest(OnMergeRequestEvent),
}

impl GitlabEvent {
    pub fn new_merged(on_merged_operations: Vec<Operation>) -> Self {
        let event = OnMergeRequestEvent { created: vec![], updated: vec![], conflict: vec![], merged: on_merged_operations };
        Self::OnMergeRequest(event)
    }
}

impl From<(String, config::Value)> for GitlabEvent {
    fn from((key, config_value): (String, Value)) -> Self {
        let operations_root = "operations";
        match key.as_str() {
            "on-comment" => {
                let on_comment_def_list = settings::array::get_array_def(config_value.clone(), operations_root).unwrap();
                Self::OnComment(event_from::def_list_to_operations(on_comment_def_list))
            }
            "on-pipeline" => event_from::pipeline(config_value, operations_root),
            "on-merge-request" => event_from::merge_req(config_value, operations_root),
            other => panic!(format!("event with name {:?} is not implemented", other))
        }
    }
}

mod event_from {
    use crate::settings;
    use config::Value;
    use crate::service::definitions::events::{GitlabEvent, OnMergeRequestEvent, OnPipelineEvent};
    use crate::service::definitions::operations::Operation;

    pub fn def_list_to_operations(list: Vec<(String, Value)>) -> Vec<Operation> {
        log::info!("def_list_to_operations");
        list.into_iter()
            .map(|e| Operation::from(e))
            .collect()
    }

    pub fn merge_req(value: Value, operations_root: &str) -> GitlabEvent {
        let merged_res = settings::array::get_array_def(value.clone(), operations_root);
        let table_res = value.into_table();
        match (merged_res, table_res) {
            (Ok(merged), Err(_)) => {
                GitlabEvent::new_merged(def_list_to_operations(merged))
            }
            (Err(_), Ok(table)) => {
                let parser_by_key = |key: &str| {
                    let def_list = table.get(key).ok_or(config::ConfigError::NotFound(key.to_string()))
                        .and_then(|value| settings::array::get_array_def(value.clone(), operations_root))
                        .unwrap_or(vec![]);
                    def_list_to_operations(def_list)
                };

                let created = {
                    let key = "created";
                    parser_by_key(key)
                };

                let updated = {
                    let key = "updated";
                    parser_by_key(key)
                };

                let conflict = {
                    let key = "conflict";
                    parser_by_key(key)
                };

                let merged = {
                    let key = "merged";
                    parser_by_key(key)
                };
                let event = OnMergeRequestEvent { created, updated, conflict, merged };
                GitlabEvent::OnMergeRequest(event)
            }
            other => unimplemented!()
        }
    }

    pub fn pipeline(value: Value, operations_root: &str) -> GitlabEvent {
        let merged_res = settings::array::get_array_def(value.clone(), operations_root);
        let table_res = value.into_table();
        match (merged_res, table_res) {
            (Ok(merged), Err(_)) => {
                GitlabEvent::new_merged(def_list_to_operations(merged))
            }
            (Err(_), Ok(table)) => {
                let parser_by_key = |key: &str| {
                    let def_list = table.get(key).ok_or(config::ConfigError::NotFound(key.to_string()))
                        .and_then(|value| settings::array::get_array_def(value.clone(), operations_root))
                        .unwrap_or(vec![]);
                    def_list_to_operations(def_list)
                };

                let started = {
                    let key = "started";
                    parser_by_key(key)
                };

                let success = {
                    let key = "success";
                    parser_by_key(key)
                };

                let failed = {
                    let key = "failed";
                    parser_by_key(key)
                };

                let event = OnPipelineEvent { started, success, failed };
                GitlabEvent::OnPipeline(event)
            }
            other => unimplemented!()
        }
    }
}

pub enum Events {
    Gitlab(Vec<GitlabEvent>)
}

impl Default for Events {
    fn default() -> Self {
        Events::Gitlab(vec![])
    }
}

impl Events {
    pub fn get_on_comment(&self) -> Vec<Operation> {
        match self {
            Events::Gitlab(gitlab_events) => {
                gitlab_events.iter()
                    .filter_map(|gitlab_event| {
                        match gitlab_event {
                            GitlabEvent::OnComment(ops) => {
                                Some(ops.clone())
                            }
                            _ => None
                        }
                    }).next()
                    .unwrap_or(vec![])
            }
        }
    }
    pub fn get_on_pipeline(&self) -> OnPipelineEvent {
        match self {
            Events::Gitlab(gitlab_events) => {
                gitlab_events.iter()
                    .filter_map(|gitlab_event| {
                        match gitlab_event {
                            GitlabEvent::OnPipeline(ops) => Some(ops.clone()),
                            _ => None
                        }
                    }).next()
                    .unwrap_or(Default::default())
            }
        }
    }

    pub fn get_on_merged(&self) -> OnMergeRequestEvent {
        match self {
            Events::Gitlab(gitlab_events) => {
                gitlab_events.iter()
                    .filter_map(|gitlab_event| {
                        match gitlab_event {
                            GitlabEvent::OnMergeRequest(ops) => Some(ops.clone()),
                            _ => None
                        }
                    }).next()
                    .unwrap_or(Default::default())
            }
        }
    }
}

impl From<config::Value> for Events {
    fn from(config_value: Value) -> Self {
        config_value.clone().into_table()
            .and_then(|v| v.get("gitlab").map(|it| it.clone()).ok_or(config::ConfigError::NotFound("gitlab".to_string())))
            .map(|it| it.clone())
            .map(|gitlab_value| {
                gitlab_value.into_table()
                    .map(|gitlab_table| {
                        let gitlab_events = gitlab_table.into_iter()
                            .map(|entry| GitlabEvent::from(entry))
                            .collect();
                        Events::Gitlab(gitlab_events)
                    }).unwrap_or(Events::default())
            }).unwrap()
    }
}