use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArgoWorkflowCreate {
    pub namespace: String,
    pub server_dry_run: bool,
    pub workflow: Workflow,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Workflow {
    pub metadata: Metadata,
    pub spec: Spec,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    pub generate_name: Option<String>,
    pub namespace: Option<String>,
    pub labels: Option<indexmap::IndexMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Spec {
    pub templates: Vec<Template>,
    pub entrypoint: String,
    pub arguments: Option<Arguments>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Template {
    pub name: String,
    pub arguments: Option<Arguments>,
    pub inputs: Option<Inputs>,
    pub outputs: Option<Outputs>,
    pub metadata: Option<Metadata>,
    pub container: Container,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Arguments {
    artifacts: Option<Vec<Artifact>>,
    parameters: Option<Vec<Parameters>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Inputs {
    artifacts: Option<Vec<Artifact>>,
    parameters: Option<Vec<Parameters>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Artifact {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Parameters {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Outputs {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Container {
    pub name: Option<String>,
    pub image: String,
    pub command: Vec<String>,
    pub args: Vec<String>,
    pub resources: Option<Resources>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Resources {}
