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


fn main() {
    size_scope_and_assignment();
    println!();
    ownership_and_functions();
}
