fn loop_results_in_assignment() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

fn loop_labels() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}


// \/ this took some time.
// '&' is the borrow symbol?
// VS Code + rust-analyzer tooltip for str
// led me to write it like so:
const LIFTOFF: &'static str = "LIFTOFF!!!";
// Looks like the next chapter, 4, is about ownership :)

fn loop_countdown_while() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("{LIFTOFF}");
}

fn loop_countdown_for() {
    for number in (1..4).rev() {
        println!("{number}");
    }
    println!("{LIFTOFF}")
}


const A_COLLECTION: [i32; 5] = [10, 20, 30, 40, 50];

fn collection_while() {
    // let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", A_COLLECTION[index]);

        index += 1;
    }
}

fn collection_for() {
    // let a = [10, 20, 30, 40, 50];

    for element in A_COLLECTION {
        println!("the value is: {element}");
    }
}

fn main() {
    // loop_results_in_assignment();
    // loop_labels();
    loop_countdown_while();
    loop_countdown_for();
    // collection_while();
    // collection_for();
}
