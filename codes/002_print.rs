fn main() {
    print!("Hello, ");
    eprintln!("An error occurred: Invalid input");
    let name = "Jhon";
    let age = 30;
    let message = format!("My name is {} and I am {} years old.", name, age);
    println!("{}", message);
    println!("Hello, {}", name)
}