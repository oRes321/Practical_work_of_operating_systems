#[test]

fn main() {
    // Use as many approaches as you can to make it work
    let x = String::from("Hello world");
    let y = &x; // x.clone();
    println!("{}, {}",x, y);
}