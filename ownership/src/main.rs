fn main() {

    // Fixed size, not changeable.
    // let s = String::from("good-bye!");

    let mut s = String::from("hello");

    // Changeable here is mutable, editable, not just replaceable.
    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{s}"); // This will print `hello, world!`

    // 4.1 Scope and Assignment
}
