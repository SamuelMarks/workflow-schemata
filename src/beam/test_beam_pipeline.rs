use crate::beam::pipeline::*;

#[test]
fn it_beam_simple_filter_and_combine_exemplifies() {
    let actual = Beam {
        pipeline: Pipeline {
            transforms: vec![
                Transform {
                    type_field: String::from("ReadFromCsv"),
                    name: String::from("ReadInputFile"),
                    config: Config {
                        path: Some(String::from(
                            "gs://apache-beam-samples/beam-yaml-blog/products.csv",
                        )),
                        language: None,
                        keep: None,
                        group_by: None,
                        combine: None,
                    },
                    input: None,
                },
                Transform {
                    type_field: String::from("Filter"),
                    name: String::from("FilterWithCategory"),
                    config: Config {
                        path: None,
                        language: Some(String::from("python")),
                        keep: Some(String::from("category == \"Electronics\"")),
                        group_by: None,
                        combine: None,
                    },
                    input: Some(String::from("ReadInputFile")),
                },
                Transform {
                    type_field: String::from("Combine"),
                    name: String::from("CountNumberSold"),
                    config: Config {
                        path: None,
                        language: None,
                        keep: None,
                        group_by: Some(String::from("product_name")),
                        combine: Some(Combine {
                            num_sold: NumSold {
                                value: String::from("product_name"),
                                fn_field: String::from("count"),
                            },
                            total_revenue: TotalRevenue {
                                value: String::from("price"),
                                fn_field: String::from("sum"),
                            },
                        }),
                    },
                    input: Some(String::from("FilterWithCategory")),
                },
                Transform {
                    type_field: String::from("WriteToCsv"),
                    name: String::from("WriteOutputFile"),
                    config: Config {
                        path: Some(String::from("output")),
                        language: None,
                        keep: None,
                        group_by: None,
                        combine: None,
                    },
                    input: Some(String::from("CountNumberSold")),
                },
            ],
        },
    };
    const BEAM_SIMPLE_FILTER_AND_COMBINE_TOML: &'static str =
        include_str!("../../beam/simple_filter_and_combine.toml");
    const BEAM_SIMPLE_FILTER_AND_COMBINE_JSON: &'static str =
        include_str!("../../beam/simple_filter_and_combine.json");
    let expect: Beam = toml::from_str(BEAM_SIMPLE_FILTER_AND_COMBINE_TOML).unwrap();
    let expect_from_json: Beam = serde_json::from_str(BEAM_SIMPLE_FILTER_AND_COMBINE_JSON).unwrap();
    assert_eq!(expect_from_json, expect);
    assert_eq!(actual, expect)
}
