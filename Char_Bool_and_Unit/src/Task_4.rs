#[test]

fn main () {
    let f:bool = true;
    let t:bool = true || false;
    assert_eq!(t,f);

    println!("Successe!");
}