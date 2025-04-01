
// Add generic for Val to make the code work, DON'T modify the code in `main`.
struct Val <T>{
    val: T,
}

impl<T: ToString>  Val<T> {
    fn value(&self) -> String {
        self.val.to_string()
    }
}

#[test]
fn main() {
    let x = Val{ val: 3.0 };
    let y = Val{ val: "hello".to_string()};
    println!("{}, {}", x.value(), y.value());
}