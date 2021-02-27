use serde::{Serialize, Deserialize};

#[serde(rename_all = "camelCase")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectDto {
    pub ring_id: String,
    pub short_name: String,
    pub leader: Leader,
    pub is_demo: bool,
    pub plugins: Plugins,
    pub name: String,
    pub id: String,
}

#[serde(rename_all = "camelCase")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Leader {
    pub id: String,
}

#[serde(rename_all = "camelCase")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plugins {
    #[serde(rename = "$type")]
    pub type_field: String,
    pub vcs_integration_settings: ProjectVcsIntegrationSettings,
    pub time_tracking_settings: ProjectTimeTrackingSettings,
}

#[serde(rename_all = "camelCase")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectVcsIntegrationSettings {
    pub processors: Vec<::serde_json::Value>,
    #[serde(rename = "$type")]
    pub type_field: String,
}

#[serde(rename_all = "camelCase")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectTimeTrackingSettings {
    pub time_spent: ::serde_json::Value,
    pub enabled: bool,
    pub estimate: ::serde_json::Value,
}
