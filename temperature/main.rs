
// take input {f32}{f|c} for conversion
// display converted temperature {f32}{F|C}

use std::io;

fn main() {
    println!("TEMPERATURE CONVERSION");

    let mut input = String::new();

    println!("Current temperature:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read temperature");

    let (degrees, unit) = input.trim().split_at(input.len() - 2);
    let degrees: f32 = degrees.parse().unwrap();
    let unit: char = unit.parse().unwrap();

    // println!("unit: {}", unit);
    // println!("degrees: {}", degrees);

    if unit == 'f' {
        println!("result: {:.1}C", f_to_c(degrees));
    } else if unit == 'c' {
        println!("result: {:.1}F", c_to_f(degrees));
    } else {
        println!("Not a valid unit");
    }


}

fn c_to_f(degrees: f32) -> f32 {
    (degrees * (9.0 / 5.0)) + 32.0
}

fn f_to_c(degrees: f32) -> f32 {
    (degrees - 32.0) * (5.0 / 9.0)
}
