// FILL in the blanks
#[test]
fn main() {
    let mut s = String::from("hello, world");

    let slice1:&str  = &s; // In two ways    2)s.as_str()
    assert_eq!(slice1, "hello, world");

    let slice2 = &s[0..5] ;
    assert_eq!(slice2, "hello");

    let mut slice3  = s ;
    slice3.push('!');
    assert_eq!(slice3, "hello, world!");


    println!("Success!");
}