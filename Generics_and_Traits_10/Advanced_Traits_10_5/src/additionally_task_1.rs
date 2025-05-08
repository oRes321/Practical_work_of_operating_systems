#[derive(Debug)]
struct VecWrapper(Vec<String>);

trait Container {
    type Item;

    fn add(&mut self, item: Self::Item);
    fn get(&self, index: usize) -> Option<&Self::Item>;
}

impl Container for VecWrapper {

    type Item = String;

    fn add(&mut self, item: Self::Item){
        self.0.push(item)
    }
    fn get(&self, index: usize) -> Option<&Self::Item>{
        self.0.get(index)
    }

}
#[test]
fn main(){
    let mut con = VecWrapper(Vec::new());
    con.add("Hello".to_string());
    println!("{:?}", con);
}