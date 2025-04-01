
// FIX the error with least changing
// DON'T remove any code line
#[test]
fn main() {
    let mut v = String::from("hello,");
    let r = &mut v;

    match  *r {
        ref mut  value =>  value.push_str(  " world!")
    }
}