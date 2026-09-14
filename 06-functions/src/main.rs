fn main() {
    println!("Hello, world!");

    another_function(5,'h');

    let x = return_five();

    println!("Return five = {x}");
}

fn another_function(x: i32, y: char) {
    println!("Another function with parameter: x = {x} and y = {y}");
}

fn return_five() -> i32 {
    5
}