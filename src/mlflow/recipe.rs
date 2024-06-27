use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MlFlowRecipe {
    pub recipe: String,
    pub target_col: String,
    pub primary_metrics: String,
    pub steps: Steps,
    pub custom_metrics: Vec<CustomMetric>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Steps {
    pub ingest: String,
    pub split: Split,
    pub transform: Transform,
    pub train: Train,
    pub evaluate: Evaluate,
    pub register: Register,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Split {
    pub split_ratios: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Transform {
    pub using: String,
    pub transformer_method: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Train {
    pub using: String,
    pub estimator_method: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Evaluate {
    pub validation_criteria: Vec<ValidationCriterum>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ValidationCriterum {
    pub metric: String,
    pub threshold: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Register {
    pub allow_non_validated_model: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CustomMetric {
    pub name: String,
    pub function: String,
    pub greater_is_better: bool,
}
