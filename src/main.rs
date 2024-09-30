trait DisplayInfo {
    fn display(&self);
    // Adding a common print_info method to the trait
    fn print_info(&self) {
        self.display(); // Default implementation
    }
}

struct Person {
    name: String,
    age: u32,
}

impl DisplayInfo for Person {
    fn display(&self) {
        println!("Name: {}, Age: {}", self.name, self.age);
    }
}

struct Dog {
    name: String,
    breed: String,
}

impl DisplayInfo for Dog {
    fn display(&self) {
        println!("Dog Name: {}, Breed: {}", self.name, self.breed);
    }
}

fn print_info(item: &dyn DisplayInfo) {
    item.display();
}

fn main() {
    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };

    let dog = Dog {
        name: String::from("Buddy"),
        breed: String::from("Golden Retriever"),
    };

    // functional polymorphism:
    print_info(&person);
    print_info(&dog);

    // OOP polymorphism:
    person.print_info();
    dog.print_info();
}