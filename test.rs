struct Person<'a> {
    name: &'a str,
}

impl<'a> Person<'a> {
    fn new(name: &'a str) -> Person<'a> {
        Person { name }
    }

    fn say_hello(&self) {
        println!("Hello, {}!", self.name);
    }
}

fn main() {
    let name = String::from("Alice");
    let person = Person::new(&name);
    person.say_hello();
    println!("Name: {}", name);
}
