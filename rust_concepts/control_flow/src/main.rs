fn main() {
    // if statements

    let number = 3;

    // the expression provided as condition must be boolean (true or false) otherwisee it will throw errors.

    if number < 5 {
        println!("{number} Less than 5");
    } else if number == 3 {
        println!("{number} is 3");
    } else {
        println!("{number} is less than 5")
    }

    // RUST SPECIFIC!!!
    // we can use if statements as an expression (code which has a return value)

    let condition = false;

    let x = if condition { 1 } else { 0 }; // else condition is necessary here

    println!("value of  x is: {x}");
}
