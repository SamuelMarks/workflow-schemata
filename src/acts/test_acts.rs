use acts::Workflow;

#[test]
fn it_module0_exemplifies() {
    let actual = Workflow::new();
    const ACTS_MODEL0: &'static str = include_str!(
        "../../acts/model0.toml"
    );
    let expect: Workflow = toml::from_str(ACTS_MODEL0).unwrap();

    assert_eq!(actual, expect)
}
