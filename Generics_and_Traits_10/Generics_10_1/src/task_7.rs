
// Fix the errors to make the code work.
struct Point<T> {
    x: T,
    y: T,
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
#[test]
fn main() {
    let p = Point{x: 5.2, y: 10.3};
    println!("{}",p.distance_from_origin());
}