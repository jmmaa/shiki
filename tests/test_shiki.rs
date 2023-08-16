use shiki::parser::number::number;

#[test]
fn test_crate() {
    let value = number(b"-0.1", 0).unwrap().0;

    println!("{value:?}");

    assert_eq!(value, b"-0.1")
}
