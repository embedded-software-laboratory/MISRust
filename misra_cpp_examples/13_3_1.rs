// Define a trait with a default implementation for a method
trait Greet {
    fn greet(&self) {
        println!("Hello from the default implementation!");
    }
}

// Implement the trait for a struct without overriding the default method
struct Person;
impl Greet for Person {}

// Implement the trait for another struct and override the default method
struct Dog;
impl Greet for Dog {
    // Overwrite has no specifier
    fn greet(&self) {
        println!("Woof! Woof!");
    }
}

fn main() {
    let person = Person;
    let dog = Dog;

    // Call the greet method on both instances
    person.greet(); // Outputs: Hello from the default implementation!
    dog.greet();    // Outputs: Woof! Woof!
}