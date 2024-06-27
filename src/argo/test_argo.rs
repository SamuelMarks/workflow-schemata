use crate::argo::workflow_create::*;

#[test]
fn it_basic_exemplifies() {
    let actual = ArgoWorkflowCreate {
        namespace: String::from("argo"),
        server_dry_run: false,
        workflow: Workflow {
            metadata: Metadata {
                generate_name: Some(String::from("hello-world-")),
                namespace: Some(String::from("argo")),
                labels: Some(indexmap::indexmap! {
                    String::from("workflows.argoproj.io/completed") => String::from("false"),
                }),
            },
            spec: Spec {
                templates: vec![Template {
                    name: String::from("whalesay"),
                    arguments: None,
                    inputs: None,
                    outputs: None,
                    metadata: None,
                    container: Container {
                        name: None,
                        image: String::from("docker/whalesay:latest"),
                        command: vec![String::from("cowsay")],
                        args: vec![String::from("hello world")],
                        resources: None,
                    },
                }],
                entrypoint: String::from("whalesay"),
                arguments: None,
            },
        },
    };
    const ARGO_WORKFLOW_CREATE: &'static str = include_str!("../../argo/workflow_create.toml");
    let expect: ArgoWorkflowCreate = toml::from_str(ARGO_WORKFLOW_CREATE).unwrap();

    assert_eq!(actual, expect)
}
