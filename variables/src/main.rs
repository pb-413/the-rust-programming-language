// Variables and Mutability

fn mutability() {
    // let x = 5; // Will not compile; later, x is set to 6!
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}

fn shadowing() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}

fn main() {
    println!("Mutability");
    mutability();

    println!();

    println!("Shadowing:");
    shadowing();
}
