use std::collections::HashMap;
use grok::{Grok, Pattern};
use std::sync::Arc;

pub struct GrokService {
    custom_patterns: HashMap<String, String>,
    merge_request_title_pattern: Arc<grok::Pattern>,
}

impl GrokService {
    pub fn new(patterns: HashMap<String, String>, merge_request_title_pattern: String) -> GrokService {
        let custom_patterns = patterns.clone();
        let mut grok = Grok::with_patterns();
        for (key, value) in patterns {
            log::info!(r###"Adding custom grok pattern. Key: "{}", Value: "{}""###, key, value);
            grok.insert_definition(key.to_uppercase(), value)
        }

        let merge_request_title_pattern = {
            let pattern = grok.compile(merge_request_title_pattern.as_str(), false).unwrap();
            Arc::new(pattern)
        };
        GrokService { custom_patterns, merge_request_title_pattern }
    }

    pub async fn get_merge_request_title_pattern(&self) -> Arc<Pattern> {
        self.merge_request_title_pattern.clone()
    }
}

#[cfg(test)]
mod test {
    use grok::Grok;

    #[test]
    fn check_mr_title() {
        let title_example = "2429 Gift Card belongs to deleted categories after disabling/enabling them";

        let mut grok = Grok::with_patterns();
        grok.insert_definition("TASK", r"[\d]+");
        grok.insert_definition("TITLE", r".+");

        let pattern = grok.compile("%{TASK} %{TITLE}", false)
            .expect("Error while compiling!");

        match pattern.match_against(title_example) {
            Some(m) =>
                println!(r#"Task: "{:?}"\n Title: "{:?}""#,
                         m.get("TASK"),
                         m.get("TITLE")),
            // Some(m) => println!("Found username {:?}", m.get("USERNAME")),
            None => println!("No matches found!"),
        }
    }
}