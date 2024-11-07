// in rust we have 3 types of loops -> for, while and loop

fn main() {
    // Infinite loop
    // loop {
    //     println!("Hello From Main");
    // }

    let mut number = 0;
    println!("value of number is {number}");

    loop {
        println!("Hello From Main");

        if number == 10 {
            break;
        }

        number = number + 1;
    }

    println!("This is an end of loop");

    // loop labels

    let mut count = 0;
    'first_loop: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'first_loop;
            }
            remaining -= 1;
        }

        count += 1;
    }

    // while loops with conditions
    let mut new_number = 10;
    while new_number != 0 {
        println!("{new_number}");

        new_number -= 1;
    }

    println!("LIFTOFF!!!");

    // for loops

    let array = [10, 20, 30, 40, 50];

    for x in array {
        print!("{x} ");
    }
}
