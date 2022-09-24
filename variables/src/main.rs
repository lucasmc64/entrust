fn main() {
    // Immutability
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // Constants
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Constant: {THREE_HOURS_IN_SECONDS}");

    // Shadowing
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x ins: {x}");

    let spaces = "    ";
    let spaces = spaces.len();
    println!("Spaces: {spaces}");

    let mut mutable_string = "Testing";
    println!("{mutable_string}");
    mutable_string = "Test completed";
    println!("{mutable_string}");
}
