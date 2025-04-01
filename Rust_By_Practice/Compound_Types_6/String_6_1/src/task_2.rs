#[test]


// Fix the error with at least two solutions
fn main() {
    let s: Box<str> = "hello, world".into();
    greetings(&s) //&*s
}

fn greetings(s: &str) {
    println!("{}",s)
}