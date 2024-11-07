use std::io;

fn main() {
    println!("Enter the temperature in Fahrenheit: ");

    let mut temp = String::new();

    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to take input");

    let result = fah_to_cel(&temp);

    println!("{}", result);
}

fn fah_to_cel(fah: &str) -> f64 {
    let float_fah: f64 = fah.trim().parse().expect("Please enter a valid number");

    let cel = (float_fah - 32.0) * 5.0 / 9.0;

    cel
}
