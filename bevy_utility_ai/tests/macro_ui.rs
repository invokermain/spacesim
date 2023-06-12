// Tests for failing macro builds from bevy_utility_macros
#[test]
fn macro_ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/*.rs")
}
