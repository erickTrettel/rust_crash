// Variables hold primitive data or references to data
// Variables are immutable by default
// Rsut is a block-soped language

pub fn run() {
    let name = "Erick";
    let mut age = 19;
    println!("My name is {} and I'm {}", name, age);

    age = 20;
    println!("My name is {} and I'm {}", name, age);

    // Define constants
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign multiple vars
    let (my_name, my_age) = ("Erick", 19);
    print!("{} is {}", my_name, my_age);
}