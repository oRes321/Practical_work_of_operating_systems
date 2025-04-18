trait Bird: std::any::Any {
    fn quack(&self);
    fn as_any(&self) -> &dyn std::any::Any;
}


struct Duck;
impl Duck {
    fn fly(&self) {
        println!("Look, the duck is flying")
    }
}
struct Swan;
impl Swan {
    fn fly(&self) {
        println!("Look, the duck.. oh sorry, the swan is flying")
    }
}

impl Bird for Duck {
    fn quack(&self) {
        println!("{}", "duck duck");
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Bird for Swan {
    fn quack(&self) {
        println!("{}", "swan swan");
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

struct  Penguin;

impl Penguin {
   fn fly(&self){
       println!("Can't fly")
   }

}

impl Bird for Penguin{
    fn quack(&self) {
        println!("Penguin")
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

#[test]
fn main() {
    // FILL in the blank to make the code work.C
    let birds: Vec<Box<dyn Bird>> = hatch_many_birds();


    for bird in birds {
        bird.quack();

        if let Some(duck) = bird.as_any().downcast_ref::<Duck>() {
            duck.fly();
            println!("This is a Duck.");
        } else if let Some(swan) = bird.as_any().downcast_ref::<Swan>() {
            swan.fly();
            println!("This is a Swan.");
        } else if bird.as_any().downcast_ref::<Penguin>().is_some() {
            println!("This is a Penguin.");
        }

    }
}

fn hatch_many_birds() -> Vec<Box<dyn Bird>> {

    let a: Vec<Box<dyn Bird>>= vec![
        Box::new(Duck),
        Box::new(Swan),
        Box::new(Penguin),
    ];
    a
}