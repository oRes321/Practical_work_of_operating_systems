#[test]

fn main() {
    let s = give_ownership();
    println!("{}", s);
}

// Only modify the code below!
fn give_ownership() -> String {
    let s = String::from("Hello world");
    // Convert String to Vec
    let s = s.into_bytes();
    let s = format!("{:?}", s);
    s

}