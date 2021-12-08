// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run(){
    // Name cannot be changed, is immutable.
    let name = "foo";
    // Age can be changed since it is mutable.
    let mut age = 16;
    println!("My name is {} and I am {}", name, age);

    // Define a constant [consts are usually all uppercase]
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign multiple vars
    let (month, year) = ("August", 2000);
    println!("{} is {}", month, year);
}