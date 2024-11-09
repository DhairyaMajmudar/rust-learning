// structs are like defining types and interfaces in typescript

// defining struct
struct User {
    username: String,
    active: bool,
    email: String,
    sign_in_count: u64,
}

// tuple structs
struct Color(u8, u8, u8);
struct Point(i32, i32, i32);

fn main() {
    // creating struct instance
    // let user1 = User {
    //     username: String::from("Dhairya Majmudar"),
    //     active: true,
    //     email: String::from("majmudar777@gmail.com"),
    //     age: 19,
    //     sign_in_count: 1,
    // };
    // println!("{}", user1.email);

    // a better way to create multiple struct instance is by using functions

    let user_1 = build_user(
        String::from("Dhairya"),
        String::from("majmudar777@gmail.com"),
    );

    // println!("{}", user_1.username);

    // Creating Instances from Other Instances with Struct Update Syntax

    // let user_2 = User {
    //     active: user_1.active,
    //     username: user_1.username,
    //     email: String::from("another@example.com"),
    //     sign_in_count: user_1.sign_in_count,
    // };

    // better way //

    let user_2 = User {
        email: String::from("new@gmail.com"),
        active: false,
        ..user_1 // the .. syntax will move the other values as it is from user_1
    };

    // tuple structs

    let red = Color(100, 0, 0);
    set_bg_color(red);

    let point = Point(-10, 20, 30);
    set_point(point);
}

fn build_user(username: String, email: String) -> User {
    User {
        // username: username,
        // email: email,
        username, // shorthand if key and value are same
        email,
        active: true,
        sign_in_count: 0,
    }
}

fn set_bg_color(color: Color) {
    println!(
        "Setting bg color to R={} G={} B={}",
        color.0, color.1, color.2
    );
}

fn set_point(point: Point) {
    println! {
        "Setting point to x={} y={} z={}",
        point.0,point.1,point.2
    };
}
