#[test]


// Fix error with at least two solutions
fn main() {
    let s = "hello, world";
    greetings(s.to_string())  //into() //String::from(s)
}

//&str
fn greetings(s: String) {
    println!("{}", s)
}