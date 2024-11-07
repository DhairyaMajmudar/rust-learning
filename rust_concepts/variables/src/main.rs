fn main() {
    // variables are immutable by default
    // mutable variables egs:
    let mut x = 5;
    println!("The value of x is: {}", x);

    x = 4;
    println!("The value of x is: {}", x);

    x = 3;
    println!("The value of x is: {}", x);

    // constants are always immutable they can't be made mutable musing mut :)
    // const must be having a type with it and it can be decalred in any scope (out of fn main function)
    const PI: f32 = 3.14;
    println!("{PI}");

    // shadowing
    // we can use the same variable multiple times by shadowing it

    let apple = 10;

    let _apple = 20;

    println!("Apples are: {apple}");
}
