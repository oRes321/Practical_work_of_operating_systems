
enum Foo {
    Bar,
    Baz,
    Qux(u32)
}
#[test]
fn main() {
    let a = Foo::Qux(10);

    // Remove the codes below, using `match` instead
    match  a {
        Foo::Bar => println!("match foo::bar"),
        Foo::Baz => println!("match foo::baz"),
        _ => println!("match others"),
    }
}