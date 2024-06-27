/// Other valid crates for to experiment with:
/// - https://crates.io/crates/action-validator
/// - https://crates.io/crates/ghactions
/// - https://crates.io/crates/ghactions-derive
/// - https://crates.io/crates/ghactions-core
/// And for actually implementing GitHub Actions (backend):
/// - https://crates.io/crates/gha-runner
use github_actions_models::workflow::{Trigger, Workflow};

#[test]
fn it_github_actions_pip_audit_ci_exemplifies() {
    let actual = Workflow::from(Workflow {
        name: None,
        run_name: None,
        on: Trigger::BareEvents(vec![]),
        permissions: Default::default(),
        env: Default::default(),
        defaults: None,
        concurrency: None,
        jobs: Default::default(),
    });
    /*
    let actual = Workflow {
        name: Some(String::from("CI")),
        run_name: None,
        on: Events(
            Box::new ({
                branch_protection_rule: Missing,
                check_run: Missing, check_suite: Missing,
                discussion: Missing, discussion_comment: Missing,
                issue_comment: Missing, issues: Missing,
                label: Missing,
                merge_group: Missing,
                milestone: Missing, project: Missing,
                project_card: Missing,
                project_column: Missing,
                pull_request: Missing,
                pull_request_comment: Missing,
                pull_request_review: Missing,
                pull_request_review_comment: Missing,
                pull_request_target: Missing,
                push: Body(
                    Push {
                        branch_filters: Some(Branches(["main"])),
                        path_filters: None, tag_filters: None
                    }
                ),
                registry_package: Missing,
                release: Missing,
                repository_dispatch: Missing,
                schedule: Body([Cron { cron: "0 12 * * *" }]),
                watch: Missing,
                workflow_call: Missing,
                workflow_dispatch: Missing,
                workflow_run: Missing
            }),
        permissions: Base(Default),
        env: {},
        defaults: None,
        concurrency: None,
        jobs: {"test": NormalJob(NormalJob { name: None, permissions: Base(Default), needs: [], if: None, runs_on: Target(["ubuntu-latest"]), environment: None, concurrency: None, outputs: {}, env: {}, defaults: None, steps: [Step { id: None, if: None, name: None, timeout_minutes: None, continue_on_error: Literal(false), body: Uses { uses: "actions/checkout@v4.1.1", with: {} } }, Step { id: None, if: None, name: None, timeout_minutes: None, continue_on_error: Literal(false), body: Uses { uses: "actions/setup-python@v5", with: {"cache-dependency-path": String("pyproject.toml"), "cache": String("pip"), "python-version": String("${{ matrix.python }}")} } }, Step { id: None, if: None, name: Some("test"), timeout_minutes: None, continue_on_error: Literal(false), body: Run { run: "make test PIP_AUDIT_EXTRA=test", working_directory: None, shell: None, env: {} } }], timeout_minutes: None, strategy: Some(Strategy { matrix: Literal(Matrix { include: Literal([]), exclude: Literal([]), dimensions: Literal({"python": [String("3.8"), String("3.9"), String("3.10"), String("3.11"), String("3.12")]}) }), fail_fast: None, max_parallel: None }), continue_on_error: Literal(false), container: None, services: {} })} };
    */
    const GITHUB_ACTIONS_PIP_AUDIT_CI: &'static str =
        include_str!("../../github_actions/pip-audit-ci.toml");
    let expect: Workflow = toml::from_str(GITHUB_ACTIONS_PIP_AUDIT_CI).unwrap();

    assert_eq!(actual, expect)
}
