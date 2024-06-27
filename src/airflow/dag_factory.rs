use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AirflowDagFactory {
    #[serde(rename = "example_dag1")]
    pub example_dag1: ExampleDag1,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExampleDag1 {
    #[serde(rename = "default_args")]
    pub default_args: DefaultArgs,
    #[serde(rename = "schedule_interval")]
    pub schedule_interval: String,
    pub concurrency: i64,
    #[serde(rename = "max_active_runs")]
    pub max_active_runs: i64,
    #[serde(rename = "dagrun_timeout_sec")]
    pub dagrun_timeout_sec: i64,
    #[serde(rename = "default_view")]
    pub default_view: String,
    pub orientation: String,
    pub description: String,
    #[serde(rename = "on_success_callback_name")]
    pub on_success_callback_name: String,
    #[serde(rename = "on_success_callback_file")]
    pub on_success_callback_file: String,
    #[serde(rename = "on_failure_callback_name")]
    pub on_failure_callback_name: String,
    #[serde(rename = "on_failure_callback_file")]
    pub on_failure_callback_file: String,
    pub tasks: Tasks,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DefaultArgs {
    pub owner: String,
    #[serde(rename = "start_date")]
    pub start_date: String,
    #[serde(rename = "end_date")]
    pub end_date: String,
    pub retries: i64,
    #[serde(rename = "retry_delay_sec")]
    pub retry_delay_sec: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tasks {
    #[serde(rename = "task_1")]
    pub task_1: Task1,
    #[serde(rename = "task_2")]
    pub task_2: Task2,
    #[serde(rename = "task_3")]
    pub task_3: Task3,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Task1 {
    pub operator: String,
    #[serde(rename = "bash_command")]
    pub bash_command: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Task2 {
    pub operator: String,
    #[serde(rename = "bash_command")]
    pub bash_command: String,
    pub dependencies: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Task3 {
    pub operator: String,
    #[serde(rename = "bash_command")]
    pub bash_command: String,
    pub dependencies: Vec<String>,
}
