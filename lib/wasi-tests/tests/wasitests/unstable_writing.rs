// !!! THIS IS A GENERATED FILE !!!
// ANY MANUAL EDITS MAY BE OVERWRITTEN AT ANY TIME
// Files autogenerated with cargo build (build/wasitests.rs).

#[test]
fn test_unstable_writing() {
    assert_wasi_output!(
        "../../wasitests/unstable/writing.wasm",
        "unstable_writing",
        vec![],
        vec![
            (
                "act1".to_string(),
                ::std::path::PathBuf::from("wasitests/test_fs/hamlet/act1")
            ),
            (
                "act2".to_string(),
                ::std::path::PathBuf::from("wasitests/test_fs/hamlet/act2")
            ),
            (
                "act1-again".to_string(),
                ::std::path::PathBuf::from("wasitests/test_fs/hamlet/act1")
            ),
        ],
        vec![],
        "../../wasitests/writing.out"
    );
}
