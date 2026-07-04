mod common;

use adder::add;

// the `tests` directory can be used for integration tests, it is treated as a completely separate
// compiled collection. Each file is treated as a crate and doesn't require the `cfg` part
// NOTE: this type of testing is only available in library crates, and not binary crates

#[test]
fn it_adds_two() {
    common::setup();

    let result = add(2, 2);
    assert_eq!(result, 4);
}