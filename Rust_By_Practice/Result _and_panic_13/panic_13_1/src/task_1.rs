

// FILL the blank
#[test]
fn drink(beverage: &str) {
    if beverage == "lemonade" {
        println!("Success!");
        // IMPLEMENT the below code
        panic!();
    }

    println!("Exercise Failed if printing out this line!");
}

fn main() {
    drink("lemo");

    println!("Exercise Failed if printing out this line!");
}