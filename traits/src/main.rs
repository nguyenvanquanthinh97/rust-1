mod trait_parameters;
mod into;
mod drop;
mod operator_overloading;
mod static_dispatch;

trait Animal
{
    fn create(name: &'static str) -> Self;
    // specify all inherited class must override this function
    fn name(&self) -> &'static str;

    // default function
    fn talk(&self)
    {
        println!("{} cannot talk", self.name());
    }
}

struct Human
{
    name: &'static str
}

impl Animal for Human
{
    fn create(name: &'static str) -> Human
    {
        Human {name}
    }

    fn name(&self) -> &'static str
    {
        self.name
    }

    fn talk(&self)
    {
        println!("{} says Hello", self.name());
    }
}

struct Cat
{
    name: &'static str
}

// impl Cat
// {
//     fn test()
//     {
//         println!("test");
//     }
// }

impl Animal for Cat
{
    fn create(name: &'static str) -> Cat
    {
        Cat {name}
    }

    fn name(&self) -> &'static str
    {
        self.name
    }

    fn talk(&self)
    {
        println!("{} says Meow Meow", self.name());
    }
}

trait Summable <T>
{
    fn sum(&self) -> T;
}

impl Summable<i32> for Vec<i32>
{
    fn sum(&self) -> i32
    {
        let mut result:i32 = 0;

        for i in self
        {
            result += *i;
        }

        return result;
    }
}

fn traits()
{
    let h = Human {name: "Thinh"};
    let c = Cat {name: "Tom"};
    let h2:Human = Animal::create("Anderson");

    h.talk();
    c.talk();
    h2.talk();

    let vec_1 = vec![1,3,6];
    println!("sum of vec_1 = {}", vec_1.sum());
}

fn main() {
    // traits();
    // trait_parameters::trait_parameters();
    // into::into();
    // drop::drops();
    // operator_overloading::main();
    static_dispatch::main();
}
