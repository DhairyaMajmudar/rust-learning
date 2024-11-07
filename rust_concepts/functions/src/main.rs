fn main() {
    another_function();
    parameter_function(10);

    let y = function_return(5, 5);

    println!("value of y is {y}");
}

fn another_function() {
    let a = 10;
    println!("Value of a is {}", a);
    println!("Another function call :)");
}

// function with parameters
fn parameter_function(a: i32) {
    println!("The value of a is {a}");
}

// statements are actions which don't return any value
// expressions are actions which do return any value

fn function_return(x: i32, y: i32) -> i32 {
    x + y // w/o ; this value will return

    // return x + y;  both statements are same
}
