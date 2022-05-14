fn main() {
    let dog = Dog {};
    let cat = Cat {};
    show_animal_data(dog);
    show_animal_data(cat);
}

trait Animal {
    fn lifespan(&self) -> u32;
    fn scientific_name(&self) -> String;
}

struct Dog;

impl Animal for Dog {
    fn lifespan(&self) -> u32 {
        10
    }

    fn scientific_name(&self) -> String {
        "Dog..".to_string()
    }
}

struct Cat;

impl Animal for Cat {
    fn lifespan(&self) -> u32 {
        20
    }

    fn scientific_name(&self) -> String {
        "Cat..".to_string()
    }
}

// <T: Animal> means type T can be used for "Animal" trait interface
fn show_animal_data<T: Animal>(val: T){
    println!("{}", val.lifespan());
    println!("{}", val.scientific_name());
}