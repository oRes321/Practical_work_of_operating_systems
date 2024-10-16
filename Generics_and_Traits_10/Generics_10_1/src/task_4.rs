
// Modify this struct to make the code work
struct Point<T,S>{
    x:T,
    y:S,
}

#[test]
fn main() {
    // DON'T modify this code.
    let p = Point{x: 5, y : "hello".to_string()};

    println!("Success!");
}