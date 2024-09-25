#[test]
// Fix the error with the use of define_x
fn main() {
    let x = "Hello";
    println!("{}, world", x);
}

fn define_x() {
     let x ="hello";
}