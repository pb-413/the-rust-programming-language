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


// TODO: Generate the nth Fibonacci number.


// TODO: Print the lyrics to the Christmas carol
// “The Twelve Days of Christmas,”
// taking advantage of the repetition in the song.


fn main() {
    test_convert_to_celsius();
}
