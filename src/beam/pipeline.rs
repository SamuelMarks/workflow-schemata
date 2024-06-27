use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Beam {
    pub pipeline: Pipeline,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pipeline {
    pub transforms: Vec<Transform>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transform {
    #[serde(rename = "type")]
    pub type_field: String,
    pub name: String,
    pub config: Config,
    pub input: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub path: Option<String>,
    pub language: Option<String>,
    pub keep: Option<String>,
    #[serde(rename = "group_by")]
    pub group_by: Option<String>,
    pub combine: Option<Combine>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Combine {
    #[serde(rename = "num_sold")]
    pub num_sold: NumSold,
    #[serde(rename = "total_revenue")]
    pub total_revenue: TotalRevenue,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NumSold {
    pub value: String,
    #[serde(rename = "fn")]
    pub fn_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TotalRevenue {
    pub value: String,
    #[serde(rename = "fn")]
    pub fn_field: String,
}
