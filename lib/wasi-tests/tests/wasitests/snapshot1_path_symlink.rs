// !!! THIS IS A GENERATED FILE !!!
// ANY MANUAL EDITS MAY BE OVERWRITTEN AT ANY TIME
// Files autogenerated with cargo build (build/wasitests.rs).

#[test]
fn test_snapshot1_path_symlink() {
    assert_wasi_output!(
        "../../wasitests/snapshot1/path_symlink.wasm",
        "snapshot1_path_symlink",
        vec![],
        vec![
            (
                "temp".to_string(),
                ::std::path::PathBuf::from("wasitests/test_fs/temp")
            ),
            (
                "hamlet".to_string(),
                ::std::path::PathBuf::from("wasitests/test_fs/hamlet")
            ),
        ],
        vec![],
        "../../wasitests/path_symlink.out"
    );
}
