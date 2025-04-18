
// Fix the errors
enum Number {
    Zero,
    One,
    Two,
}

enum Number1 {
    Zero = 0,
    One,
    Two,
}

// C-like enum
enum Number2 {
    Zero = 0,
    One = 1,
    Two = 2,
}

#[test]
fn main() {
    // An enum variant can be converted to a integer by `as`
    assert_eq!(Number::One as i32, Number1::One as i32);
    assert_eq!(Number1::One as i32 , Number2::One as i32 );

    println!("Success!");
}