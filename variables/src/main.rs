fn main() {
    // variables
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // constants
    const THIS_IS_NINE: u32 = 9;
    println!("The value of const: {THIS_IS_NINE}");
    
    // shadowing
    let s = 5;
    let s = s + 1;

    {
        let s = s * 2;
        println!("The value of s in the inner scope is: {s}");
    }

    println!("The value of x is: {s}");

    let x = 2.0; // f64

    let y: f32 = 3.0; // f32

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    let f: bool = false; // with explicit type annotation

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    // tupples
    let tup: (i32, f64, u8) = (500, 6.4, 1); // tupples

    let five_hundred = tup.0;

    let six_point_four = tup.1;

    let one = tup.2;

    // arrays
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];

    use std::io;

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}