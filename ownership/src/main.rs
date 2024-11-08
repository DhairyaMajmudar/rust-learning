// in rust memory is managed through the system of ownership
// size known -> put it on stack (more orgaised LIFO and fast process)
// size unknown -> put it on heap (less organised and slow to process data)

// dynamaic memory allocation is done through heap
// known memory allocation is done through stack

// the main purpose of ownership is to manage heap data

// ownership rules
// 1. Each value in rust has an owner
// 2. There can only be one owner at a time
// 3. when owner goes out of scope the value will be dropped

// &str -> This will store on stack, not growable
// String -> This will store on heap, growable

// MUST READ!!! https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#variables-and-data-interacting-with-move

fn main() {
    let mut s = String::from("Dhairya"); // string stored in heap and its now growable

    s.push_str(" Majmudar");

    {
        // new scope
        let mut s_new = String::from("New String");
        s_new.push_str(" Hello :))");

        println!("{s_new}");
    }

    // println!("{}", s_new); // this will throw error as s_new is out of scope
    println!("{s}");

    move_example();

    let latest_s = gives_ownership(); // here we changed the owner
    println!("{latest_s}");

    let s1 = String::from("Dhairya Majmudar");

    let lenght = reference(&s1);
    println!("{lenght}");
}

fn move_example() {
    let s1: String = String::from("Dhairya from cool things!!");

    let s2 = s1; // this will move the ownership of s1 to s2

    // expensive operation
    // let s2 = s1.clone() // this will clone the s1 and create a new copy of s1 on s2

    // println!("{s1}"); // this will throw error as s1 is moved to s2 hence s1 is no longer valid
    println!("{s2}");
}

fn gives_ownership() -> String {
    let s = String::from("Hello from gives_ownership function");
    s
}

// !!References and Borrowing!!
// references are represented by "&" symbol
// references are like pointers where we can pass the address of the data stored in memory

fn reference(s: &String) -> usize {
    // here we are not passing ownership which thus it will not be dropped after function scope ends
    s.len()
}
