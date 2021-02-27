use serde::{Serialize, Deserialize};
use crate::rest_api::json_models::issue::field::{ProjectCustomFieldType, ProjectCustomField, FieldColor};
use crate::rest_api::json_models::issue::field::custom_field::{IssueCustomField, StateIssueCustomField};
use crate::rest_api::json_models::issue::field::value::User;
use crate::rest_api::json_models::project::ProjectDto;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IssueDto {
    // pub resolved: Option<u32>,
    pub summary: Option<String>,
    pub number_in_project: Option<u8>,
    pub reporter: Option<IssueReporter>,
    pub id_readable: Option<String>,
    // voters
    pub has_email: Option<bool>,
    pub event_source_ticket: String,
    // attachments
    pub wikified_description: String,
    // updater
    // comments
    // externalIssue
    pub hidden_attachments_count: u8,
    // applicableActions
    pub is_draft: bool,
    // visibility
    pub description: Option<String>,
    pub tags: Option<Vec<IssueTagDto>>,
    // created
    // links
    pub project: ProjectDto,
    pub uses_markdown: bool,
    // updated
    // watchers
    pub fields: Vec<IssueCustomField>,
    pub id: String,
    #[serde(alias = "$type")]
    pub model_type: String,
}

impl IssueDto {
    pub fn get_state(&self) -> Option<&StateIssueCustomField> {
        self.fields.iter()
            .filter_map(|field| match field {
                IssueCustomField::StateIssueCustomField(stateIssueCustomField) =>
                    Some(stateIssueCustomField),
                _ => None
            })
            .next()
    }

    pub fn get_state_field(&self) -> Option<StateIssueCustomField> {
        self.fields.iter()
            .filter_map(|field| match field {
                IssueCustomField::StateIssueCustomField(data) => Some(data.clone()),
                _ => None
            })
            .next()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IssueReporter {
    ring_id: String,
    online: bool,
    // profiles
}

#[cfg(test)]
pub mod tests {
    use std::fs::File;
    use std::io::BufReader;
    use crate::rest_api::json_models::issue::IssueDto;

    #[test]
    fn test_it() {
        let result1 = std::env::current_dir().unwrap();
        println!("{:?}", result1.as_os_str());
        let file = File::open("src/rest_api/json_models/issue.json")
            .or(File::open("sdk/youtrack-tools/src/rest_api/json_models/issue.json"))
            .unwrap();
        let reader = BufReader::new(file);

        let result: IssueDto = serde_json::from_reader(reader).unwrap();
        println!("{:?}", result)
    }
}

pub mod field {
    use serde::{Serialize, Deserialize, Deserializer, Serializer};
    use crate::rest_api::json_models::issue::field::custom_field::IssueCustomField::StateIssueCustomField;
    use crate::rest_api::json_models::issue::field::custom_field::IssueCustomField;
    use serde_json::Value;
    use serde::de::{Error, Unexpected};

    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct FieldBundle {
        id: String,
        #[serde(alias = "$type")]
        model_type: String,
    }

    #[serde(tag = "valueType")]
    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub enum FieldType {
        // SimpleString,
        // SimpleText,
        // SimpleDate,
        // SimpleDateTime,
        // SimplePeriod,
        #[serde(alias = "integer")]
        SimpleInteger,
        // SimpleFloat,

        #[serde(alias = "enum")]
        EnumeratedEnum,
        // EnumeratedGroup,
        #[serde(alias = "user")]
        EnumeratedUser,
        // EnumeratedOwnerField,
        #[serde(alias = "state")]
        EnumeratedState,
        #[serde(alias = "version")]
        EnumeratedVersion,
        // EnumeratedBuild,
        #[serde(other)]
        OtherType,
        // {
        //     #[serde(alias = "$type")]
        //     model_type: String
        // },
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    // #[serde(tag = "$type", rename_all = "camelCase")]
    #[serde(tag = "$type")]
    pub enum Field {
        #[serde(rename_all = "camelCase")]
        CustomField {
            localized_name: Option<String>,
            field_type: FieldType,
            ordinal: u8,
            name: Option<String>,
            id: String,
        }
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct ProjectCustomField {
        pub bundle: Option<FieldBundle>,
        pub empty_field_text: Option<String>,
        pub is_public: bool,
        pub ordinal: Option<u8>,
        pub can_be_empty: bool,
        pub field: Field,
        pub id: String,
        #[serde(alias = "$type")]
        pub custom_field_type: ProjectCustomFieldType,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub enum ProjectCustomFieldType {
        EnumProjectCustomField,
        UserProjectCustomField,
        SimpleProjectCustomField,
        StateProjectCustomField,
        #[serde(other)]
        OtherType,
    }

    // #[derive(Serialize, Deserialize, Debug, Clone)]
    // // #[serde(tag = "$type", rename_all = "camelCase")]
    // #[serde(tag = "$type")]
    // pub enum ProjectCustomField {
    //     #[serde(rename_all = "camelCase")]
    //     EnumProjectCustomField {
    //         bundle: Option<FieldBundle>,
    //         empty_field_text: Option<String>,
    //         is_public: bool,
    //         ordinal: u8,
    //         can_be_empty: bool,
    //         field: Field,
    //         id: String,
    //     },
    //     #[serde(rename_all = "camelCase")]
    //     UserProjectCustomField {
    //         bundle: Option<FieldBundle>,
    //         empty_field_text: Option<String>,
    //         is_public: bool,
    //         ordinal: u8,
    //         can_be_empty: bool,
    //         field: Field,
    //         id: String,
    //     },
    //     #[serde(rename_all = "camelCase")]
    //     SimpleProjectCustomField {
    //         empty_field_text: Option<String>,
    //         is_public: bool,
    //         ordinal: u8,
    //         can_be_empty: bool,
    //         field: Field,
    //         id: String,
    //     },
    //     #[serde(rename_all = "camelCase")]
    //     StateProjectCustomField{
    //         bundle: Option<FieldBundle>,
    //         empty_field_text: Option<String>,
    //         is_public: bool,
    //         ordinal: u8,
    //         can_be_empty: bool,
    //         field: Field,
    //         id: String,
    //     },
    //     #[serde(other)]
    //     OtherType,
    //     // {
    //     //     #[serde(alias = "$type")]
    //     //     model_type: String
    //     // },
    // }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[serde(tag = "$type")]
    pub enum FieldColor {
        #[serde(rename_all = "camelCase")]
        FieldStyle {
            id: String
        }
    }

    pub mod value {
        use crate::rest_api::json_models::issue::field::{FieldColor, IssueStateType};
        use serde::{Serialize, Deserialize};
        use serde;

        #[serde(rename_all = "camelCase")]
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct EnumBundleElement {
            localized_name: Option<String>,
            archived: bool,
            color: Option<FieldColor>,
            name: Option<String>,
            id: String,
        }

        #[serde(rename_all = "camelCase")]
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct StateBundleElement {
            pub is_resolved: bool,
            pub localized_name: Option<String>,
            pub archived: bool,
            pub color: Option<FieldColor>,
            pub name: Option<IssueStateType>,
            pub id: String,
            pub ordinal: Option<u8>,
        }

        #[serde(rename_all = "camelCase")]
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct User {
            ring_id: String,
            avatar_url: Option<String>,
            login: Option<String>,
            full_name: Option<String>,
            name: Option<String>,
            id: Option<String>,
        }

        #[serde(rename_all = "camelCase")]
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct VersionBundleElement {
            archived: bool,
            color: Option<FieldColor>,
            name: Option<String>,
            id: String,
        }

        #[derive(Serialize, Deserialize, Debug, Clone)]
        #[serde(tag = "$type")]
        // #[serde(tag = "$type", rename_all = "camelCase")]
        pub enum FieldValue {
            EnumBundleElement(EnumBundleElement),
            StateBundleElement(StateBundleElement),
            User(User),
            VersionBundleElement(VersionBundleElement),
            #[serde(other)]
            OtherType,
            // {
            //     #[serde(alias = "$type")]
            //     model_type: String
            // },
        }
    }

    pub mod custom_field {
        use serde::{Serialize, Deserialize};
        use crate::rest_api::json_models::issue::field::{ProjectCustomField, IssueStateType};
        use crate::rest_api::json_models::issue::field::value::{FieldValue, StateBundleElement};

        #[serde(rename_all = "camelCase")]
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct SingleEnumIssueCustomField {
            project_custom_field: ProjectCustomField,
            value: Option<FieldValue>,
            is_updatable: bool,
            name: String,
            id: String,
        }

        #[serde(rename_all = "camelCase")]
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct SingleUserIssueCustomField {
            project_custom_field: ProjectCustomField,
            value: Option<FieldValue>,
            is_updatable: bool,
            name: String,
            id: String,
        }

        #[serde(rename_all = "camelCase")]
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct MultiVersionIssueCustomField {
            project_custom_field: ProjectCustomField,
            value: Vec<FieldValue>,
            is_updatable: bool,
            name: String,
            id: String,
        }

        #[serde(rename_all = "camelCase")]
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct SimpleIssueCustomField {
            project_custom_field: ProjectCustomField,
            value: Option<FieldValue>,
            is_updatable: bool,
            name: String,
            id: String,
        }

        #[serde(rename_all = "camelCase")]
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct StateIssueCustomField {
            pub project_custom_field: ProjectCustomField,
            pub value: FieldValue,
            pub is_updatable: bool,
            pub name: String,
            pub id: String,
        }

        impl StateIssueCustomField {
            pub fn field_id(&self) -> String {
                self.project_custom_field.bundle.as_ref().unwrap()
                    .id.clone()
            }

            pub fn status_id(&self) -> String {
                match &self.value {
                    FieldValue::StateBundleElement(StateBundleElement {
                                                       id: state_id,
                                                       name: Some(state_name),
                                                       ..
                                                   }) => Some(state_id.clone()),
                    _ => None
                }.unwrap()
            }

            pub fn state_name(&self) -> IssueStateType {
                match &self.value {
                    FieldValue::StateBundleElement(StateBundleElement {
                                                       id: state_id,
                                                       name: Some(state_name),
                                                       ..
                                                   }) => Some(state_name.clone()),
                    _ => None
                }.unwrap()
            }
        }

        #[derive(Serialize, Deserialize, Debug, Clone)]
        #[serde(tag = "$type")]
        pub enum IssueCustomField {
            #[serde(rename_all = "camelCase")]
            SingleEnumIssueCustomField(SingleEnumIssueCustomField),
            #[serde(rename_all = "camelCase")]
            SingleUserIssueCustomField(SingleUserIssueCustomField),
            #[serde(rename_all = "camelCase")]
            MultiVersionIssueCustomField(MultiVersionIssueCustomField),
            #[serde(rename_all = "camelCase")]
            SimpleIssueCustomField(SimpleIssueCustomField),
            #[serde(rename_all = "camelCase")]
            StateIssueCustomField(StateIssueCustomField),
        }

        pub type IssueStatus = IssueCustomField;
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum IssueStateType {
        Submitted,
        Open,
        InProgress,
        WaitForMerge,
        Fixed,
        ReadyForTesting,
        Verified,
        TBD,
        Duplicate,
        Reopened,
        WaitForDesignReview,
        DesignAccepted,
        Obsolete,
        Done,
        ToVerify,
        Other(String),
    }

    impl IssueStateType {
        pub fn new(state_name: &str) -> Self {
            match state_name.to_lowercase().as_str() {
                "submitted" => IssueStateType::Submitted,
                "open" => IssueStateType::Open,
                "in progress" => IssueStateType::InProgress,
                "wait for merge" => IssueStateType::WaitForMerge,
                "fixed" => IssueStateType::Fixed,
                "ready for testing" => IssueStateType::ReadyForTesting,
                "verified" => IssueStateType::Verified,
                "tbd" => IssueStateType::TBD,
                "duplicate" => IssueStateType::Duplicate,
                "reopened" => IssueStateType::Reopened,
                "wait for design review" => IssueStateType::WaitForDesignReview,
                "design accepted" => IssueStateType::DesignAccepted,
                "obsolete" => IssueStateType::Obsolete,
                "done" => IssueStateType::Done,
                "to verify" => IssueStateType::ToVerify,
                other => IssueStateType::Other(other.to_string())
            }
        }
    }

    impl Serialize for IssueStateType {
        fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error> where S: Serializer {
            let text_value: String = Into::into(self.clone());
            serializer.serialize_str(text_value.as_str())
        }
    }

    impl<'de> Deserialize<'de> for IssueStateType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de>, {
            let value = <Value as Deserialize>::deserialize(deserializer)?;

            let result = value.as_str().ok_or({
                D::Error::invalid_value(
                    Unexpected::Seq, &format!("Wrong state type: {:?}", &value).as_str())
            }).map(|value| IssueStateType::new(value))
                .map_err(|err| {
                    D::Error::invalid_value(
                        Unexpected::Other("web hook"),
                        &format!("{:?}", err).as_str(),
                    )
                });
            result
        }
    }

    impl Into<String> for IssueStateType {
        fn into(self) -> String {
            match self {
                IssueStateType::Submitted => "submitted".to_string(),
                IssueStateType::Open => "open".to_string(),
                IssueStateType::InProgress => "in progress".to_string(),
                IssueStateType::WaitForMerge => "wait for merge".to_string(),
                IssueStateType::Fixed => "fixed".to_string(),
                IssueStateType::ReadyForTesting => "ready for testing".to_string(),
                IssueStateType::Verified => "verified".to_string(),
                IssueStateType::TBD => "tbd".to_string(),
                IssueStateType::Duplicate => "duplicate".to_string(),
                IssueStateType::Reopened => "reopened".to_string(),
                IssueStateType::WaitForDesignReview => "wait for design review".to_string(),
                IssueStateType::DesignAccepted => "design accepted".to_string(),
                IssueStateType::Obsolete => "obsolete".to_string(),
                IssueStateType::Done => "done".to_string(),
                IssueStateType::ToVerify => "to verify".to_string(),
                IssueStateType::Other(value) => value.clone(),
            }
        }
    }
}

#[serde(rename_all = "camelCase")]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IssueTagDto {
    pub is_usable: Option<bool>,
    pub color: Option<FieldColor>,
    // owner:
    pub query: Option<String>,
    pub is_updatable: Option<bool>,
    pub is_shareable: Option<bool>,
    pub name: String,
    pub id: Option<String>,
    #[serde(alias = "$type")]
    #[serde(rename = "$type")]
    pub model_type: String,
}

impl IssueTagDto {
    pub fn new(name: String, style: String) -> IssueTagDto {
        let color = Some(style).map(|style_id| FieldColor::FieldStyle { id: style_id });
        let model_type = "IssueTag".to_string();
        IssueTagDto { name, color, model_type, ..IssueTagDto::default() }
    }
}

#[deprecated(note = "use issue::IssueTagDto")]
pub mod tag {
    use crate::rest_api::json_models::issue::field::value::User;
    use crate::rest_api::json_models::issue::field::FieldColor;
    use serde::{Serialize, Deserialize};

    #[serde(rename_all = "camelCase")]
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Root {
        pub tag_sharing_settings: Option<TagSharingSettings>,
        pub untag_on_resolve: Option<bool>,
        pub is_usable: bool,
        pub color: FieldColor,
        pub owner: Option<User>,
        pub query: String,
        pub is_updatable: bool,
        pub is_deletable: Option<bool>,
        pub is_shareable: Option<bool>,
        pub read_sharing_settings: Option<TagSharingSettings>,
        pub update_sharing_settings: Option<TagSharingSettings>,
        pub issues_url: Option<String>,
        pub pinned: Option<bool>,
        pub name: String,
        pub id: String,
    }

    #[serde(rename_all = "camelCase")]
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct TagSharingSettings {
        pub permission_based_tag_access: bool,
        pub permitted_groups: Vec<::serde_json::Value>,
        pub permitted_users: Vec<::serde_json::Value>,
    }

    #[serde(rename_all = "camelCase")]
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct WatchFolderSharingSettings {
        pub permitted_groups: Vec<::serde_json::Value>,
        pub permitted_users: Vec<::serde_json::Value>,
    }
}