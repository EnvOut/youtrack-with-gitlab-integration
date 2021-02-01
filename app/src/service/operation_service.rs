use crate::service::definitions::operations::Operation;
use crate::service::definitions::operations::change_task::ChangeTasks;
use crate::service::definitions::args::Argument;
use crate::service::definitions::filter_args::FilterArgs;
use crate::service::Service;
use crate::service::youtrack_service::YoutrackService;
use crate::service::grok_service::GrokService;
use crate::service::pattern_builder_service::mustache::MustachePatternBuilderService as PatternService;
use crate::service::definitions::base::{Filter, UpdateKind};
use youtrack_tools::rest_api::issue::search::IssueSearchParam;
use youtrack_tools::rest_api::client::YoutrackClient;
use crate::service::definitions::tags::TagDefinition;

#[derive(new)]
pub struct OperationService {
    youtrack_service: Service<YoutrackService>,
    builder_service: Service<PatternService>,
    grok_service: Service<GrokService>,
}

impl OperationService {
    pub async fn call_operations(&self, operations: Vec<Operation>, args: Argument) {
        for operation in operations {
            self.call_operation(operation, &args).await
        }
    }

    pub async fn call_operation(&self, operation: Operation, args: &Argument) {
        match operation {
            Operation::ChangeTasks(
                ChangeTasks {
                    name:operation_name,
                    // args,
                    custom_args,
                    update,
                    filter: hook_filters,
                    filter_args: FilterArgs { equals, has }
                }) => {
                let merged_args = custom_args + args;

                let has_all_args = merged_args.has_all(&has);
                let all_args_equals = merged_args.all_equals(&equals);

                log::info!("Operation: {:?}, checking conditions: has_all_args - {}, all_args_equals - {}", operation_name, has_all_args, all_args_equals);

                if has_all_args && all_args_equals {
                    log::info!("Started operation: {:?}", operation_name);

                    let issueSearchParams = hook_filters.iter().cloned()
                        .map(|it| it.into())
                        .collect::<Vec<IssueSearchParam>>();

                    let mut youtrack_service = self.youtrack_service.write().await;
                    let client = youtrack_service.get_client().await;
                    let issues = youtrack_service.find_issues(issueSearchParams).await
                        .unwrap_or(vec![]);

                    log::info!("operation: {:?} found filtered issues: {}", operation_name, issues.len());

                    for mut issue in issues {
                        for  update_kind in &update {
                            match update_kind {
                                kind @ UpdateKind::Status(_) => issue.set_state(kind.to_state()),
                                UpdateKind::AddTag(TagDefinition {
                                                       name, title, style
                                                   }) => {
                                    youtrack_service.add_configured_tag(issue.project_id.clone(), issue.id.clone(), (title.clone(), style.to_string())).await;
                                }
                                UpdateKind::Title(title) => issue.description = Some(title.clone())
                            }
                        }
                        ()
                    }
                };

                ()
            }
        }
    }
}