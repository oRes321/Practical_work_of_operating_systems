
trait Action{
     fn execute(&self) -> String;
}


struct SayHello;
struct SayBye;
struct PanicAction;
struct SayRust;


impl Action for SayHello{
    fn execute(&self) -> String {
        "Hello".to_string()
    }
}
impl Action for SayBye {
    fn execute(&self) -> String {
        "Bye".to_string()
    }
}

impl Action for PanicAction {
    fn execute(&self) -> String {
        "Panic".to_string()
    }

}

impl Action for SayRust{
    fn execute(&self) -> String {
        "Rust is awesome".to_string()
    }
}

#[test]
fn main(){
    let  say:Vec<Box<dyn Action>> = vec![
        Box::new(SayHello),
        Box::new(SayBye),
        Box::new(PanicAction),
        Box::new(SayRust)
    ];

    for a in say {
        println!("{}",a.execute());
    }


}