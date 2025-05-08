struct RangePair(i32, i32);

trait Summarizable {
    type Start;
    type End;

    fn start(&self) -> Self::Start;
    fn end(&self) -> Self::End;
}


impl Summarizable for RangePair{
    type Start = i32;
    type End = i32;

    fn start(&self) -> i32{ self.0 }
    fn end(&self) -> i32 {self.1}

}
#[test]
fn main() {
    let pair = RangePair(3, 5);

    println!("Start: {}", pair.start());
    println!("End: {}", pair.end());
}


