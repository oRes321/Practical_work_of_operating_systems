use std::str;
// FILL in the blanks
#[test]
fn main() {
    let mut s = String::new();
    s = "hello".to_string();

    // Some bytes, in a vector
    let v = vec![104, 101, 108, 108, 111];

    // Turn a byte's vector into a String
    let s1 = str::from_utf8(&v).unwrap().to_string();


    assert_eq!(s, s1);

    println!("Success!");
}