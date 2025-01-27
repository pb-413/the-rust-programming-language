fn main() {
    println!("Hello, world!");

    another_function();

    another_function_param(42);

    let value = 5;
    let unit_label = 'h';
    print_labeled_measurement(value, unit_label);

    let x = five();
    println!("The value of x is: {x} (should be five)");

    let x = plus_one(x);
    println!("x plus one is: {x}");
}

fn another_function() {
    println!("Another function.");
}

fn another_function_param(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
