use config::Value;
use std::collections::HashMap;
use crate::service::definitions::error::DefinitionError;
use crate::service::definitions::operations::change_task::ChangeTasks;

pub type OperationDef = Operation;

#[derive(Clone, Debug)]
pub enum Operation {
    ChangeTasks(ChangeTasks)
}

impl From<(String, HashMap<String, Value>)> for Operation {
    fn from((default_name, definition): (String, HashMap<String, Value>)) -> Self {
        let type_name: String = definition.get("type")
            .map(|it| it.clone())
            .map(|it| it.into_str().ok())
            .flatten()
            .unwrap();
        // Operations
        let result = match type_name.clone().as_str() {
            "ChangeTasks" => {
                let operation = ChangeTasks::from((default_name, definition));
                Ok(Operation::ChangeTasks(operation))
            }
            value => Err(DefinitionError::UnsupportedType(value.to_string()))
        };
        result.unwrap()
    }
}

impl From<(String, Value)> for Operation {
    fn from((default_name, root): (String, Value)) -> Self {
        root.into_table()
            .map(|definition| {
                let type_name: String = definition.get("type")
                    .map(|it| it.clone())
                    .map(|it| it.into_str().ok())
                    .flatten()
                    .unwrap();
                // Operations
                let result = match type_name.clone().as_str() {
                    "ChangeTasks" => {
                        let operation = ChangeTasks::from((default_name, definition));
                        Ok(Operation::ChangeTasks(operation))
                    }
                    value => Err(DefinitionError::UnsupportedType(value.to_string()))
                };
                result.unwrap()
            }).unwrap()
    }
}

pub mod change_task {
    use crate::service::definitions::base::{ArgsMap, UpdateDef, Filters, Filter, UpdateKind};
    use std::collections::HashMap;
    use config::Value;
    use std::collections::hash_map::RandomState;
    use crate::service::definitions::args::{Argument};
    use crate::service::definitions::filter_args::FilterArgs;
    use crate::service::definitions::args::Argument::CustomArgs;

    #[derive(Clone, Debug)]
    pub struct ChangeTasks {
        pub name: String,
        // Argument::CustomArgs
        pub custom_args: Argument,
        // pub args: Argument,
        pub update: UpdateDef,
        pub filter: Filters,
        pub filter_args: FilterArgs,
    }

    impl From<(String, HashMap<String, Value>)> for ChangeTasks {
        fn from((default_name, definition): (String, HashMap<String, Value>)) -> Self {
            let name = definition.get("name")
                .and_then(|it| it.clone().into_str().ok())
                .unwrap_or(default_name);
            let custom_args = definition.get("custom-args")
                .and_then(|it| it.clone().into_table().ok())
                .map(|it| Argument::CustomArgs(it))
                .unwrap_or(Argument::CustomArgs(Default::default()));
            let update = definition.get("update")
                .and_then(|it| it.clone().into_table().ok())
                .map(|it| {
                    let mut update_list = it.clone().into_iter()
                        .map(|it| UpdateKind::from(it))
                        .collect::<Vec<_>>();
                    update_list.sort_by(|x, y| x.partial_cmp(y).unwrap());
                    update_list
                })
                .unwrap_or_default();
            let filter = definition.get("filter")
                .and_then(|it| it.clone().into_table().ok())
                .unwrap_or_default()
                .into_iter()
                .map(|it|Filter::from(it))
                .collect::<Vec<_>>();
            let filter_args = definition.get("filter")
                .and_then(|it| it.clone().into_table().ok())
                .map(|it| {
                    let equals = it.get("equals")
                        .and_then(|it| it.clone().into_table().ok())
                        .unwrap_or_default();
                    let has = it.get("has")
                        .and_then(|it| it.clone().into_table().ok())
                        .unwrap_or_default();
                    FilterArgs { equals, has }
                })
                .unwrap_or_default();

            ChangeTasks {
                name,
                custom_args,
                update,
                filter,
                filter_args,
            }
        }
    }
}