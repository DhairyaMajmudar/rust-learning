// learning structs
// program to calculate area of rectangle

// defining struct

#[derive(Debug)] // debug trait to print rectangle
struct Rectangle {
    height: u32,
    width: u32,
}

// method syntax https://doc.rust-lang.org/book/ch05-03-method-syntax.html#method-syntax
// these are smiliar to functions but are defined within the context of structs
// their first parameter is always self

impl Rectangle {
    fn cal_area(&self) -> u32 {
        self.height * self.width
    }
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

    // learning method syntax
    println!("area obtained through method {}", rect1.cal_area());
}

fn area(dimensions: &Rectangle) -> u32 {
    dimensions.height * dimensions.width
}
