//Program to solve the roots of an quadratic equation
use std::io;

fn main()
{
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

//Inputting the coeficients of the equation

    println!("Please enter the first coefficient: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let a:f64 = input1.trim().parse().expect("Not a valid number");

    println!("Please enter the second coefficient: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let b:f64 = input2.trim().parse().expect("Not a valid number");

    println!("Please enter the third coefficient: ");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let c:f64 = input3.trim().parse().expect("Not a valid number");

//Calculating the roots of the equation

    let rootone:f64 = (-b - (b.powi(2) - (4.0 * a * c)).sqrt()) / (2.0 * a);
    let roottwo:f64 = (-b + (b.powi(2) - (4.0 * a * c)).sqrt()) / (2.0 * a);

//Outputting the results    

    println!("The first root is, {}", rootone);
    println!("The second root is, {}", roottwo);
}