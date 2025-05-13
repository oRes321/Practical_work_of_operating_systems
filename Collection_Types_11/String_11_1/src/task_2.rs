// FILL in the blanks
#[test]
fn main() {
    let mut s = String::from("hello, world");

    let slice1: &str = "hello, world"; // In two ways
    assert_eq!(slice1, "hello, world");

    let slice2 = "hello";
    assert_eq!(slice2, "hello");

    let mut slice3: String = s;
    slice3.push('!');
    assert_eq!(slice3, "hello, world!");

    println!("Success!");
}