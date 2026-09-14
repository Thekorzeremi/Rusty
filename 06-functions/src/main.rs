fn main() {
    println!("Hello, world!");

    another_function(5,'h');
}

fn another_function(x: i32, y: char) {
    println!("Another function with parameter: x = {x} and y = {y}");
}