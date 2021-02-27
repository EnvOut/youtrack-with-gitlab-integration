pub trait PatternBuilderService {}

pub mod mustache {
    use std::collections::HashMap;
    use std::collections::hash_map::RandomState;
    use mustache::Template;

    pub struct MustachePatternBuilderService {
        youtrack_task_builder_template: Template
    }

    impl MustachePatternBuilderService {
        pub fn new() -> MustachePatternBuilderService {
            let youtrack_task_builder_template = {
                let string_template = crate::settings::get_str("gitlab.merge-request.youtrack-task-id-builder").unwrap();
                mustache::compile_str(string_template.as_str()).unwrap()
            };
            MustachePatternBuilderService { youtrack_task_builder_template }
        }


        pub fn youtrack_task_build(&self, parsed_values: HashMap<String, String>) -> String {
            let mut bytes = vec![];
            self.youtrack_task_builder_template.render(&mut bytes, &parsed_values).unwrap();
            String::from_utf8(bytes).unwrap()
        }
    }
}

