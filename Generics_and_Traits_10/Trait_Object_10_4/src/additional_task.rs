
trait Command{
    fn run(&self) -> String;
}

struct PrintHello;

struct PrintNumber(i32);
struct PrintCustom(String);

impl Command for PrintHello{
    fn run(&self) -> String{
        String::from("Hello")
    }
}

impl Command for PrintNumber{
    fn run(&self) -> String {
        format!("Number: {}", self.0)
    }
}


impl Command for PrintCustom{
    fn run(&self) -> String{
        format!("Custom: {}", self.0)
    }
}


fn run_static<T: Command>(cmd: T){
    println!("{}",cmd.run())
}
#[test]
fn main(){

    let a:Vec<Box<dyn Command>> = vec![
        Box::new(PrintHello),
        Box::new(PrintNumber(43)),
        Box::new(PrintCustom(String::from("Rustacean")))
    ];
    run_static(PrintHello);
    run_static(PrintNumber(99));
    run_static(PrintCustom("Yo!".to_string()));

    for b in a {
        println!("{}",b.run());
    }
}

