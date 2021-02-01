use std::collections::HashMap;

use config::Value;

use crate::service::definitions::error::DefinitionError;

pub mod operations;
pub mod error;
pub mod events;
pub mod tags;

pub mod base {
    use std::collections::HashMap;
    use std::vec::Vec;

    use config::{Value as ConfigValue, Value};

    use youtrack_tools::rest_api::issue::search::IssueSearchParam;
    use youtrack_tools::rest_api::json_models::issue::field::IssueStateType;

    use crate::service::definitions::tags::TagDefinition;

    pub type ArgsMap = HashMap<String, String>;
    pub type Filters = Vec<Filter>;

    #[derive(Clone, Debug)]
    pub enum Filter {
        Id(String),
        State(String),
        ProjectName(String),
        Tag(TagDefinition),
    }

    impl Into<IssueSearchParam> for Filter {
        fn into(self) -> IssueSearchParam {
            match &self {
                Filter::Id(id) => IssueSearchParam::IssueId(id.clone()),
                Filter::State(state) => IssueSearchParam::State(state.clone()),
                Filter::ProjectName(projectName) => IssueSearchParam::ProjectName(projectName.clone()),
                Filter::Tag(tag) => IssueSearchParam::TagTitle(tag.title.clone())
            }
        }
    }

    impl From<(String, config::Value)> for Filter {
        fn from((key, value): (String, Value)) -> Self {
            match key.to_lowercase().as_str() {
                "id" => {
                    value.into_str()
                        .map(|it| Filter::Id(it))
                        .unwrap()
                }
                "state" => {
                    value.into_str()
                        .map(|it| Filter::State(it))
                        .unwrap()
                }
                "project_name" => {
                    value.into_str()
                        .map(|it| Filter::ProjectName(it))
                        .unwrap()
                }
                "tag" => {
                    value.into_str()
                        .and_then(|it| crate::settings::get_tag_definition(it.as_str()))
                        .map(|it| Filter::Tag(it))
                        .unwrap()
                }
                _ => unimplemented!("Supported values only: id, state, project_id, tag")
            }
        }
    }

    pub type TagRef = String;

    // pub type UpdateDef = HashMap<String, ConfigValue>;
    pub type UpdateDef = Vec<UpdateKind>;

    #[derive(PartialOrd, Ord, PartialEq, Eq, Clone, Debug)]
    pub enum UpdateKind {
        Status(String),
        AddTag(TagDefinition),
        Title(String),
    }

    impl From<(String, config::Value)> for UpdateKind {
        fn from((key, value): (String, Value)) -> Self {
            match key.to_lowercase().as_str() {
                "add-tag" => value.into_str()
                    .and_then(|v| crate::settings::get_tag_definition(&v))
                    .map(|tag_def| UpdateKind::AddTag(tag_def))
                    .unwrap(),
                "status" => value.into_str().map(|it| UpdateKind::Status(it.clone())).unwrap(),
                "title" => value.into_str().map(|it| UpdateKind::Title(it.clone())).unwrap(),
                _ => unimplemented!()
            }
        }
    }

    impl UpdateKind {
        pub fn to_state(&self) -> IssueStateType {
            match self {
                UpdateKind::Status(status) => IssueStateType::new(status.clone().as_str()),
                UpdateKind::AddTag(_) | UpdateKind::Title(_) => unimplemented!()
            }
        }
    }


    // #[derive(Default)]
    // pub struct UpdateDef {
    //     add_tag: Option<TagRef>,
    //     title: Option<String>,
    //     state: Option<String>,
    // }
}

pub mod args {
    use std::collections::HashMap;
    use std::error::Error;
    use std::ops::Add;
    use std::result;

    use config::FileFormat;
    use serde::Serialize;

    use gitlab_tools::models::hooks::merge_request::MergeRequestHook;
    use gitlab_tools::models::hooks::note::NoteHook;
    use gitlab_tools::models::hooks::pipeline::PipelineHook;

    use crate::service::definitions::filter_args::{FilterArgsEquals, FilterArgsHas};

    #[derive(Clone, Debug)]
    pub enum Argument {
        NoteHookArgs(NoteHook),
        PipelineHookArgs(PipelineHook),
        MergeRequestHookArgs(MergeRequestHook),
        CustomArgs(HashMap<String, config::Value>),
        MergedArgs(HashMap<String, config::Value>),
    }

    impl Default for Argument {
        fn default() -> Self {
            Self::MergedArgs(Default::default())
        }
    }

    impl Argument {
        pub fn has_all(&self, has: &FilterArgsHas) -> bool {
            // match self {
            //     Self::MergedArgs(self_data) => {
            //         let by_keys = {
            //             self_data.len() >= has.len()
            //                 && has.keys().all(|k| self_data.contains_key(k))
            //         };
            //
            //         let by_values = {
            //             has.clone().into_iter()
            //                 .map(|(has_key, has_value)| {
            //                     (self_data.get(&has_key), has_value)
            //                 })
            //                 .filter(|(self_value, _)| self_value.is_some())
            //                 // TODO value == value it is wrong comparing for "has"
            //                 .all(|(self_value, has_value)| self_value.unwrap().clone() == has_value)
            //         };
            //         by_keys && by_values
            //     }
            //     _ => unimplemented!()
            // }
            // TODO should be reimplemented
            true
        }

        pub fn all_equals(&self, filter_args_equals: &FilterArgsEquals) -> bool {
            match self {
                Self::MergedArgs(self_data) => {
                    let by_keys = {
                        self_data.len() >= filter_args_equals.len()
                            && filter_args_equals.keys().all(|k| self_data.contains_key(k))
                    };

                    let by_values = {
                        filter_args_equals.clone().into_iter()
                            .map(|(equals_key, equals_value)| {
                                (self_data.get(&equals_key), equals_value)
                            })
                            .filter(|(self_value, _)| self_value.is_some())
                            .all(|(self_value, equals_value)| self_value.unwrap().clone() == equals_value)
                    };
                    by_keys && by_values
                }
                _ => unimplemented!()
            }
        }

        pub fn to_config(&self) -> Option<HashMap<String, config::Value>> {
            match self {
                Argument::NoteHookArgs(_) | Argument::PipelineHookArgs(_) | Argument::MergeRequestHookArgs(_) => {
                    let mut self_string_res = None;

                    if let (Argument::NoteHookArgs(note_hook)) = self {
                        self_string_res = serde_yaml::to_string(note_hook).ok();
                    }
                    if let (Argument::PipelineHookArgs(pipeline_hook)) = self {
                        self_string_res = serde_yaml::to_string(pipeline_hook).ok();
                    }
                    if let (Argument::MergeRequestHookArgs(merge_request_hook)) = self {
                        self_string_res = serde_yaml::to_string(merge_request_hook).ok();
                    }

                    self_string_res.map(|yaml| {
                        let mut settings = config::Config::default();
                        settings
                            .merge(config::File::from_str(&yaml, FileFormat::Yaml)).unwrap();
                        settings.cache.clone().into_table().unwrap()
                    })
                }
                Argument::CustomArgs(_) | Argument::MergedArgs(_) => {
                    let mut result = None;
                    if let Argument::CustomArgs(custom_args) = &self {
                        result = Some(custom_args.clone());
                    };
                    if let Argument::MergedArgs(merged_args) = &self {
                        result = Some(merged_args.clone());
                    };
                    result
                }
            }
        }
    }

    impl std::ops::Add<Argument> for Argument {
        type Output = Argument;

        /// return Argument::MergedArgs
        fn add(self, arg: Argument) -> Self::Output {
            self + &arg
        }
    }

    impl std::ops::Add<&Argument> for Argument {
        type Output = Argument;

        /// return Argument::MergedArgs
        fn add(self, arg: &Argument) -> Self::Output {
            if let (Some(self_config), Some(arg_config)) = (self.to_config(), arg.to_config()) {
                let mut result = HashMap::default();
                result.extend(self_config);
                result.extend(arg_config);
                Argument::MergedArgs(result)
            } else {
                unimplemented!()
            }
        }
    }
}

pub mod filter_args {
    use std::collections::HashMap;

    #[derive(Default, Debug, Clone)]
    pub struct FilterArgs {
        pub equals: FilterArgsEquals,
        pub has: FilterArgsHas,
    }

    pub type FilterArgsEquals = HashMap<String, config::Value>;
    pub type FilterArgsHas = HashMap<String, config::Value>;
}