fn size_scope_and_assignment() {
    // Fixed size, not changeable.
    // let s = String::from("good-bye!");

    let mut s = String::from("hello");

    // Changeable here is mutable, editable, not just replaceable.
    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{s}"); // This will print `hello, world!`

    // 4.1 Scope and Assignment
    // Copying a string:
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}");

    // Copying scalar values:
    let x = 5;
    let y = x;

    println!("x = {x}, y = {y}");
}


fn ownership_and_functions() {
    let s: String = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.


//Slices
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
fn second_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    let mut n1 = 0;
    let mut found_first_space = false;
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
           if !found_first_space{
                n1 = i + 1; // +1 for length of ' '
                found_first_space = true;
           } else {
                return &s[n1..i]
           }
        }
    }

    &s[n1..]
}
fn other_slices() {
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3])
}

fn main() {
    size_scope_and_assignment();
    println!();
    ownership_and_functions();
    println!();
    let s = String::from("Hello user; welcome!");
    let first = first_word(&s);
    println!("First of '{s}' is '{first}'");
    let second = second_word(&s);
    println!("Second of '{s}' is '{second}'");
    other_slices();
}
