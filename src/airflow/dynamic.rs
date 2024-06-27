use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AirflowDynamic {
    pub dag_id: String,
    pub schedule: String,
    pub bash_command: String,
    pub env_var: String,
}
