use unit_and_integration_tests;

#[test]
fn it_adds_two() {
    assert_eq!(4, unit_and_integration_tests::add(2, 2));
}

// Integration tests are put in a tests subfolder and the compiler knows to treat them then as
// tests. We can use multiple files (they will all be separate crates) and in case we need to 
// write some code that the tests should use we can create submodules by creating a new
// subdirectory with the code in tests. Integration tests are not available for binary crates since
// they are not meant to be used externaly
