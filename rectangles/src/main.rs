// learning structs
// program to calculate area of rectangle

// defining struct

#[derive(Debug)] // debug trait to print rectangle
struct Rectangle {
    height: u32,
    width: u32,
}

fn main() {
    // inistance of rectangle
    let rect1 = Rectangle {
        height: 30,
        width: 40,
    };

    let result = area(&rect1);
    println!("{}", result);

    println!("{:?}", rect1); // priting rectangle
    println!("{:#?}", rect1); // pretty priting rectagle
}

fn area(dimensions: &Rectangle) -> u32 {
    dimensions.height * dimensions.width
}
