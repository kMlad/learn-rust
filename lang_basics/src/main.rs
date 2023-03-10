fn main() {
    let _cat: &str = "Hehence";

    let _dog = String::new();
    let dog = String::from("Jack");

    let owner = format!("Hi! I'm {}, the owner of {}", "Kiril", dog);

    println!("{}", owner);

    // Invoking functions
    say_hi();
    let mut name = "John";
    say_hello_name(name);
    println!("{}", mutate(&mut name));
}
fn say_hi() {
    println!("Hello there!");
}

fn say_hello_name(name: &str) {
    println!("Hello {}", name);
}

fn mutate(name: &mut &str) -> String {
    *name = "Xavier";
    return format!("My new name is {}", name);
}
