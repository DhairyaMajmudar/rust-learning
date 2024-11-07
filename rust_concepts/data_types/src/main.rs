// Rust is statically type language
// Two kind of data types -> Scalar and Compound

// Scalar -> integers, floating-point numbers, Booleans, and characters.

// must read --> https://doc.rust-lang.org/book/ch03-02-data-types.html

// Compound -> tuples and arrays

fn main() {
    // tuple -> It is grouping of data having different data types
    // we cannot grow or shrink tuple once declared

    // the tuple w/o any  value has a name "unit"
    let unit_tup = ();
    println!("The value of unit tuple is {:?}", unit_tup);

    let tup = (250, 3.14, 12);
    println!("The value of tup is: {} {} {}", tup.0, tup.1, tup.2);
    // or //
    println!("The value of tup is: {:?}", tup);

    // destructuring tuple
    let (x, y, z) = tup;
    println!("The value of x is {}", x);

    // array -> IT is grouping of data having same data types.

    let array_a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("The vaue of a is {:?}", array_a);

    // we creating an array of same elements
    let same_array = [3; 5]; // This will create an array of 5 3's
    println!("The value of same_array is {:?}", same_array);

    println!("The value of first element of array_a is {}", array_a[0]);
    println!("The value of second element of array_a is {}", array_a[1]);
}
