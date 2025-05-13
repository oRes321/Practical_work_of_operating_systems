
// FILL in the blanks
#[test]
fn main() {
    let mut s = String::new();
    s = String::from("hello");

    // Some bytes, in a vector
    let mut v = vec![104, 101, 108, 108, 111];

    // Turn a byte's vector into a String
    let s1 = String::from_utf8(v).unwrap();


    assert_eq!(s, s1);

    println!("Success!");
}