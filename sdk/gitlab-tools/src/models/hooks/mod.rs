pub mod base;
pub mod merge_request;
pub mod note;
pub mod pipeline;
use serde::{self, Serialize, Deserialize, Deserializer};
use crate::models::hooks::pipeline::PipelineHook;
use crate::models::hooks::note::NoteHook;
use crate::models::hooks::merge_request::MergeRequestHook;
use serde_json::Value;
use serde::de::{Unexpected, Error};

#[derive(Debug, Clone)]
pub enum GitlabHookRequest {
    // merge_request
    MergeRequest(MergeRequestHook),
    // note
    Note(NoteHook),
    // pipeline
    Pipeline(PipelineHook),
}

impl<'de> Deserialize<'de> for GitlabHookRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let object = <Value as Deserialize>::deserialize(deserializer)?;

        let kind = match object.pointer("/object_kind") {
            Some(&Value::String(ref kind)) => kind,
            Some(_) => {
                return Err(D::Error::invalid_type(
                    Unexpected::Other("JSON value"),
                    &"a string",
                ));
            }
            None => {
                return Err(D::Error::missing_field("object_kind"));
            }
        }.clone();

        match kind.clone().as_str() {
            "merge_request" => serde_json::from_value(object).map(|hook| GitlabHookRequest::MergeRequest(hook)),
            "note" => serde_json::from_value(object).map(|hook| GitlabHookRequest::Note(hook)),
            "pipeline" => serde_json::from_value(object).map(|hook| GitlabHookRequest::Pipeline(hook)),
            _ => {
                return Err(D::Error::invalid_value(
                    Unexpected::Other("object kind"),
                    &format!("unrecognized webhook object kind: {}", object).as_str(),
                ));
            }
        }.map_err(|err| {
            log::info!("Can't deserialize webhook kind: {kind:?}, error: {error}", kind = &kind, error = &err);
                D::Error::invalid_value(
                    Unexpected::Other("web hook"),
                    &format!("{:?}", err).as_str(),
                )
            })
    }
}

