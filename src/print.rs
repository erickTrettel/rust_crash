pub fn run() {
    // print to console
    println!("Hello from the print.rs file");

    println!("Number: {}", 1);

    // Basic Fromatting
    println!("{} is from {}", "Erick", "SP");

    // Positional arguments
    println!(
        "{0} is from {1} and {0} likes to {2}", 
        "Erick", 
        "SP", 
        "code"
    );

    // Name arguments
    println!(
        "{name} likes to play {activity}", 
        name = "John", 
        activity = "baseball"
    );

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // Placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

    // Basic math
    println!("10 + 10 = {}", 10 + 10);
}