trait Animal {
    fn speak(&self);
}

struct Dog {
    name: String,
}

impl Animal for Dog {
    fn speak(&self) {
        println!("{} says: Woof!", self.name);
    }
}

impl Dog {
    // Associated function to create a new Dog
    pub fn new(name: &str) -> Dog {
        Dog {
            name: name.to_string(),
        }
    }
}

impl Drop for Dog {
    fn drop(&mut self) {
        println!("Dropping Dog named: {}", self.name);
    }
}

fn main() {
    // alternative direct instantiation
    let my_dog = Dog {
        name: "Buddy".to_string(),
    };
    let my_dog: Box<dyn Animal> = Box::new(Dog::new("Buddy"));
    my_dog.speak();
    // my_dog goes out of scope here, and Drop is called automatically
}