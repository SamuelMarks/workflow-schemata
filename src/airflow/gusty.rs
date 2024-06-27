use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AirflowGusty {
    pub operator: String,
    #[serde(rename = "multi_task_spec")]
    pub multi_task_spec: indexmap::IndexMap<String, String>,
}
