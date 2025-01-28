use std::collections::HashMap;

// Convert temperatures between Fahrenheit and Celsius.
fn convert_to_celsius(fahrenheit: f32) -> f32 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn test_convert_to_celsius() {
    let mut temps: HashMap<String, f32> = HashMap::new();
    temps.insert("hot".to_string(), 99.0);
    temps.insert("warm".to_string(), 72.0);
    temps.insert("cold".to_string(), 35.0);
    temps.insert("freezing".to_string(), 32.0);
    temps.insert("bitter_cold".to_string(), 0.0);

    for (label, temp) in temps {
        let c = convert_to_celsius(temp);
        println!("{label}: {temp}F {c}C");
    }
    // Surprised that the HashMap was unordered compared to the inserts.
    // Oh, not only is it un-ordered,
    // it is randomly ordered when running again.
}


// Generate the nth Fibonacci number.
fn fibonacci(n: i32) -> i32 {
    let n1 = 0;
    let n2 = 1;
    if n == 1 {
        n1
    }
    else if n == 2 {
        n2
    }
    else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

fn test_fibonacci() {
    // 0, 1, 1, 2, 3, 5, 8, 13
    // 1, 2, 3, 4, 5, 6, 7, 8
    for n in 1..9 {
        let f = fibonacci(n);
        println!("fib({n}) = {f}")
    }
}

// TODO: Print the lyrics to the Christmas carol
// “The Twelve Days of Christmas,”
// taking advantage of the repetition in the song.


fn main() {
    // test_convert_to_celsius();
    test_fibonacci();
}
