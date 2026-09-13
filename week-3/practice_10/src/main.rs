fn main() {
    // addition
    let sum = 5550 + 7310;
    println!("The sum of 5550 and 7310 = {}", sum); // Fixed: println!

    // subtraction
    let difference: f64 = 95.5 - 4.3; // Fixed: Changed type from u32 to f64 because result is a decimal
    println!("The difference of 95.5 and 4.3 = {}", difference); // Fixed: println!

    // multiplication
    let product: f32 = 4.0 * 30.0; // Fixed: Added decimals (.0) to match the f32 type
    println!("The multiple of 4 and 30 = {}", product); // Fixed: println!

    // division
    let quotient = 56.7 / 32.2;
    println!("The division of 56.7 and 32.2 = {}", quotient); // Fixed: println!

    // remainder
    let remainder = 43 % 5;
    println!("The remainder of 43 and 5 = {}", remainder); // Fixed: println!
}
