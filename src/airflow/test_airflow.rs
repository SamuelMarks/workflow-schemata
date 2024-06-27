mod test_dyn {
    use crate::airflow::dynamic::*;

    #[test]
    fn it_dynamic_exemplifies() {
        let actual = AirflowDynamic {
            dag_id: String::from("dag_file_1"),
            schedule: String::from("'@daily'"),
            bash_command: String::from("'echo $ENVVAR'"),
            env_var: String::from("'Hello! :)'"),
        };
        const AIRFLOW_DYNAMIC: &'static str = include_str!("../../airflow/dynamic.toml");
        let expect: AirflowDynamic = toml::from_str(AIRFLOW_DYNAMIC).unwrap();

        assert_eq!(actual, expect)
    }
}

mod test_gusty {
    use crate::airflow::gusty::*;

    #[test]
    fn it_gusty_exemplifies() {
        let actual = AirflowGusty {
            operator: String::from("airflow.operators.bash.BashOperator"),
            multi_task_spec: indexmap::indexmap! {
                String::from("bash_task_1") => String::from("echo first_task"),
                String::from("bash_task_2") => String::from("echo second_task"),
            },
        };
        const GUSTY_DYNAMIC: &'static str = include_str!("../../airflow/gusty.toml");
        let expect: AirflowGusty = toml::from_str(GUSTY_DYNAMIC).unwrap();

        assert_eq!(actual, expect)
    }
}

mod test_dag_factory {
    use crate::airflow::dag_factory::*;
    #[test]
    fn it_dag_factory_exemplifies() {
        let actual = AirflowDagFactory {
            example_dag1: Default::default(),
        };
        const GUSTY_DYNAMIC: &'static str = include_str!("../../airflow/dag_factory.toml");
        let expect: AirflowDagFactory = toml::from_str(GUSTY_DYNAMIC).unwrap();

        assert_eq!(actual, expect)
    }
}
