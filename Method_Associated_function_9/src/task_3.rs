struct TrafficLight {
    color: String,
}

impl TrafficLight {
    // Using `Self` to fill in the blank.
    pub fn show_state(&self)  {
        println!("the current state is {}", self.color);
    }

    // Fill in the blank, DON'T use any variants of `Self`.
    pub fn change_state(mut trafficLight:TrafficLight) {
        trafficLight.color = "green".to_string()
    }
}
#[test]
fn main() {
    println!("Success!");
}