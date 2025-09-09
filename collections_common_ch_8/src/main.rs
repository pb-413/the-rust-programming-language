// 8.1 Storing lists of values with Vectors.

fn vectors_guide() {
    //Creation
    let _v: Vec<i32> = Vec::new();

    let _v = vec![1, 2, 3];

    // Updating
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    //Reading
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2]; //Panic if index out of bound.
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2); //No panic if out of bound.
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    // Reading - Reminder re borrowing
    // let mut v = vec![1, 2, 3, 4, 5];

    // let first = &v[0];

    // v.push(6); // This fails because when we grow a vector, we can't have a mutable reference to an item in it.

    // println!("The first element is: {first}");

    // Iteration
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }
    let mut v = vec![100, 32, 42];
    for i in &mut v {
        *i += 50
    }

    // Using an Enum to store multiple types (because you know the set of types before run time)
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}

fn main() {
    // println!("Hello, world!");
    vectors_guide();
}
