// !!! THIS IS A GENERATED FILE !!!
// ANY MANUAL EDITS MAY BE OVERWRITTEN AT ANY TIME
// Files autogenerated with cargo build (build/wasitests.rs).

#[test]
fn test_unstable_quine() {
    assert_wasi_output!(
        "../../wasitests/unstable/quine.wasm",
        "unstable_quine",
        vec![std::path::PathBuf::from("."),],
        vec![],
        vec![],
        "../../wasitests/quine.out"
    );
}
