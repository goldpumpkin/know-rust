struct Cat;

struct Dog;

trait Animal {
    fn name(&self) -> &'static str;
}

impl Animal for Cat {
    fn name(&self) -> &'static str {
        "Cat"
    }
}

impl Animal for Dog {
    fn name(&self) -> &'static str {
        "Dog"
    }
}

fn name(a: impl Animal) {
    println!("Animal name:{:?}", a.name())
}

fn main() {
    let cat = Cat;
    name(cat);
}