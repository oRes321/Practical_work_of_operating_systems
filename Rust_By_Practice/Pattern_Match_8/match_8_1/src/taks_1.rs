enum Direction{
    East,
    West,
    North,
    South,
}
#[test]
fn main() {
    let dire = Direction::South;
    match dire {
        Direction::East => println!("East"),
        Direction:: South  => { // Matching South or North here
            println!("West");
        },
        _ => println!("East or West"),
    };
}