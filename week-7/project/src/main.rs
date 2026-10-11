use std::io;

fn main() {
    // Creating the menu
    println!("Select the shape your working with:\nTrapezium\nRhombus\nParallelogram\nCube\nCylinder\n");

    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    //Converting to lowercase
    let shape = input1.trim().to_lowercase();

    // Computing the units of the value
    println!("What is the unit ");

    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let unit = input2.trim();

    // Linking the input to the functions
    if shape == "trapezium" {
        trapezium(unit);
    } else if shape == "rhombus" {
        rhombus(unit);
    } else if shape == "parallelogram" {
        parallelogram(unit);
    } else if shape == "cube" {
        cube(unit);
    } else if shape == "cylinder" {
        cylinder(unit);
    } else {
        println!("Unsupported shape");
    }
}

// Making the functions for the calculator
fn trapezium(unit: &str) {
    println!("Input the value of base 1");
    let mut input3 = String::new();
    io::stdin().read_line(&mut input3).expect("Not valid float");
    let b1: f32 = input3.trim().parse().expect("Not valid number");

    println!("Input the value of base 2");
    let mut input4 = String::new();
    io::stdin().read_line(&mut input4).expect("Not valid float");
    let b2: f32 = input4.trim().parse().expect("Not valid number");

    println!("Input the value of the height");
    let mut input5 = String::new();
    io::stdin().read_line(&mut input5).expect("Not valid float");
    let h: f32 = input5.trim().parse().expect("Not valid number");

    let area: f32 = (h / 2.0) * (b1 + b2);
    println!("Area of your trapezium is {}{}^2", area, unit);
}

fn rhombus(unit: &str) {
    println!("Input the value of the first diagonal");
    let mut input6 = String::new();
    io::stdin().read_line(&mut input6).expect("Not valid float");
    let d1: f32 = input6.trim().parse().expect("Not valid number");

    println!("Input the value of the second diagonal");
    let mut input7 = String::new();
    io::stdin().read_line(&mut input7).expect("Not valid float");
    let d2: f32 = input7.trim().parse().expect("Not valid number");

    let ar: f32 = 0.5 * d1 * d2;
    println!("The area of your rhombus is {}{}^2", ar, unit);
}

fn parallelogram(unit: &str) {
    println!("Input the value of the base of your parallelogram");
    let mut input8 = String::new();
    io::stdin().read_line(&mut input8).expect("Not valid float");
    let b: f32 = input8.trim().parse().expect("Not valid number");

    println!("Input the value of the altitude of the parallelogram");
    let mut input9 = String::new();
    io::stdin().read_line(&mut input9).expect("Not valid float");
    let a: f32 = input9.trim().parse().expect("Not valid number");

    let are: f32 = a * b;
    println!("The area of your parallelogram is {}{}^2", are, unit);
}

fn cube(unit: &str) {
    println!("Input the value of the side of your cube?");
    let mut input11 = String::new();
    io::stdin().read_line(&mut input11).expect("Not valid float");
    let side: f32 = input11.trim().parse().expect("Not valid number");

    let surface_area: f32 = 6.0 * side * side;
    println!("The surface area of your cube is {}{}^2", surface_area, unit);
}

fn cylinder(unit: &str) {
    println!("Input the value of the radius of your cylinder?");
    let mut input14 = String::new();
    io::stdin().read_line(&mut input14).expect("Not valid float");
    let r: f32 = input14.trim().parse().expect("Not valid number");

    println!("Input the value of the height of your cylinder?");
    let mut input15 = String::new();
    io::stdin().read_line(&mut input15).expect("Not valid float");
    let height: f32 = input15.trim().parse().expect("Not valid number");

    // Using Rust's official internal PI constant instead of 3.142 for better rounding accuracy
    let volume: f32 = std::f32::consts::PI * r * r * height;
    println!("The volume of your cylinder is {}{}^3", volume, unit);
}
