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
}