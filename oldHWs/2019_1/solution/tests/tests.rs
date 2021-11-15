use solution::*;

#[test]
fn test_hex() {
    let inp = "dd";
    let inp2 = "7b";
    
    assert_eq!(hex(inp2), Some(0x3));
    assert_eq!(hex(inp), Some(0xb));

    println!("{:?}", octal("345"));
}

#[test]
fn test_to_string() {

    let num = 26;
    assert_eq!(convert_to(num, 16), String::from("1a"));
}

/* #[test]
fn test_octal_basic() {
    assert_eq!(octal("345"), Some(0o5));

    for n in 0..=7 {
        assert_eq!(octal(&n.to_string()), Some(n));
    }
    assert_eq!(octal("10"), Some(0o1));
} */
