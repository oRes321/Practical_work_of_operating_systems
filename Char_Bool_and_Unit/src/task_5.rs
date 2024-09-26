#[test]

fn main() {
    let _v: () = ();

    let v = (3, 3);
    assert_eq!( v , v);

    println!("Success!");
}

fn implicitly_ret_unit() {
    println!("I will return a ()");
}

fn explicitly_ret_unit() -> () {
    println!("I will return a ()");
}