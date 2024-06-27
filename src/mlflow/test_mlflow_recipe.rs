use crate::mlflow::recipe::*;

#[test]
fn it_github_actions_pip_audit_ci_exemplifies() {
    let actual = MlFlowRecipe {
        recipe: String::from("regression/v1"),
        target_col: String::from("fare_amount"),
        primary_metrics: String::from("root_mean_squared_error"),
        steps: Steps {
            ingest: String::from("${INGEST_CONFIG}"),
            split: Split {
                split_ratios: String::from("${SPLIT_RATIOS|default([0.75, 0.125, 0.125])}"),
            },
            transform: Transform {
                using: String::from("custom"),
                transformer_method: String::from("transformer_fn"),
            },
            train: Train {
                using: String::from("custom"),
                estimator_method: String::from("estimator_fn"),
            },
            evaluate: Evaluate {
                validation_criteria: vec![
                    ValidationCriterum {
                        metric: String::from("root_mean_squared_error"),
                        threshold: 10,
                    },
                    ValidationCriterum {
                        metric: String::from("weighted_mean_squared_error"),
                        threshold: 20,
                    },
                ],
            },
            register: Register {
                allow_non_validated_model: false,
            },
        },
        custom_metrics: vec![CustomMetric {
            name: String::from("weighted_mean_squared_error"),
            function: String::from("weighted_mean_squared_error"),
            greater_is_better: false,
        }],
    };
    const MLFLOW_RECIPE0: &'static str = include_str!("../../mlflow/recipe0.toml");
    let expect: MlFlowRecipe = toml::from_str(MLFLOW_RECIPE0).unwrap();

    assert_eq!(actual, expect)
}
