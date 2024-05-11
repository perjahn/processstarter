fn main() {
    // Defining a mutable variable with 'let mut'
    // Using the macro vec! to create a vector
    let mut values = vec![1, 2, 3, 4];

    for value in &values {
        println!("value = {}", value);
    }

    if values.len() > 5 {
        println!("List is longer than five items");
    }

    // Pattern matching
    match values.len() {
        0 => println!("Empty"),
        1 => println!("One value"),
        // pattern matching can use ranges of integers
        2..=10 => println!("Between two and ten values"),
        11 => println!("Eleven values"),
        // A `_` pattern is called a "wildcard", it matches any value
        _ => println!("Many values"),
    };

    // while loop with predicate and pattern matching using let
    while let Some(value) = values.pop() {
        println!("value = {value}"); // using curly brackets to format a local variable
    }
}
