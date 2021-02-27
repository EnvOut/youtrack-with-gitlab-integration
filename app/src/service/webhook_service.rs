use youtrack_tools::rest_api::client::YoutrackClientImpl;
use std::sync::Arc;
use gitlab_tools::models::hooks::GitlabHookRequest;
use crate::service::youtrack_service::YoutrackService;
use std::error::Error;
use gitlab_tools::models::hooks::merge_request::MergeRequestHook;
use gitlab_tools::models::hooks::merge_request::custom::{MergeRequestState, MergeRequestAction};
use std::borrow::BorrowMut;
use tokio::sync::RwLock;
use snafu::{ensure, Backtrace, ErrorCompat, ResultExt, Snafu};
use crate::service::Service;
use crate::service::pattern_builder_service::mustache::MustachePatternBuilderService as PatternService;
use std::collections::HashMap;
use crate::service::grok_service::GrokService;
use gitlab_tools::models::hooks::note::NoteHook;
use crate::settings;
use crate::service::definitions::events::GitlabEvent;
use crate::service::definitions::events::Events;
use crate::service::operation_service::OperationService;
use crate::service::definitions::args::Argument;

#[derive(new)]
pub struct SimpleWebhookService {
    youtrack_service: Service<YoutrackService>,
    builder_service: Service<PatternService>,
    grok_service: Service<GrokService>,
    operation_service: Service<OperationService>,
}

// #[derive(Debug, Snafu)]
#[derive(Debug)]
// #[derive(Debug, new)]
pub enum SimpleWebhookServiceError {
    // #[snafu(display("Note hook not yet implemented"))]
    NoteHookNotImplemented,
    // #[snafu(display("Pipeline hook not yet implemented"))]
    PipelineHookNotImplemented,
}

lazy_static! {
	 static ref MERGE_REQUEST_TITLE_PATTERN: grok::Pattern = {
	       let mut grok = grok::Grok::with_patterns();
        grok.insert_definition("TASK", r"[\d]+");
        grok.insert_definition("TITLE", r".+");

        grok.compile("%{TASK} %{TITLE}", false)
            .expect("Error while compiling!")
	 };
}

impl SimpleWebhookService {
    pub async fn process_web_hook(&mut self, webhook: GitlabHookRequest) -> Result<(), SimpleWebhookServiceError> {
        let result = match webhook {
            GitlabHookRequest::MergeRequest(merge_request_hook) => {
                Ok(self.process_merge_request_hook(merge_request_hook).await)
            }
            GitlabHookRequest::Note(note_hook) => {
                Ok(self.process_note_hook(note_hook).await)
            }
            GitlabHookRequest::Pipeline(_) => {
                Err(SimpleWebhookServiceError::PipelineHookNotImplemented)
            }
        };
        result
    }

    pub async fn process_note_hook(&mut self, note_hook: NoteHook) {
        let merge_request_title = note_hook.merge_request.title.clone();

        if let Some(task_id) = self.get_task_id(merge_request_title).await {
            let mut service = self.youtrack_service.write().await;
            let project_id = settings::get_str("youtrack.project_id").unwrap();
            if let Ok(events_value) = settings::get("app.events") {
                let events = Events::from(events_value);

                let on_comment_events = events.get_on_comment();

                let operation_service = self.operation_service.write().await;
                operation_service.call_operations(on_comment_events, Argument::NoteHookArgs(note_hook)).await;
                println!("events");
            }
        }
    }

    pub async fn process_merge_request_hook(&mut self, merge_request_hook: MergeRequestHook) {
        let merge_request_action = merge_request_hook.object_attributes.action;
        let task_id = self.get_task_id(merge_request_hook.object_attributes.title).await;
        let mr_state = merge_request_hook.object_attributes.state;

        if let (Some(MergeRequestAction::Merge), Some(task_id), MergeRequestState::Merged) = (merge_request_action, task_id, mr_state) {
            let mut service = self.youtrack_service.write().await;
            service.update_status(task_id.clone()).await;
        }

        if let Some(true) = merge_request_hook.object_attributes.merge_when_pipeline_succeeds {
            // set label pipeline in progress
        }
    }

    async fn get_task_id(&mut self, title: String) -> Option<String> {
        let grok_service = self.grok_service.read().await;
        match grok_service.get_merge_request_title_pattern().await.match_against(title.as_ref()) {
            Some(matches) => {
                let mut captured_patterns = HashMap::new();
                for (capture, name) in matches.iter() {
                    // println!("capture: {:?}, names: {:?}", capture, names);
                    captured_patterns.insert(capture.to_string(), name.to_string());
                }

                let task_readable_identifier = if let Some(_) = crate::settings::get_str("gitlab.merge-request.youtrack-task-id-builder").ok() {
                    self.builder_service.clone().read().await.youtrack_task_build(captured_patterns)
                } else {
                    title.clone()
                };

                Some(task_readable_identifier)
            }
            _ => None
        }
    }
}