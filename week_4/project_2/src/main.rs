use std::io;

fn main()
{
    let mut input = String::new();

    println!("If experienced, please type 1\nif not, please type 2: ");
    io::stdin().read_line(&mut input).expect("Not a valid string");
    let experienced:u8 = input.trim().parse().expect("Not a valid number");

    input.clear();

    if experienced == 2
    {
        println!("Your annual incentive is N100,000");
    }
    else if experienced == 1
    {let mut input = String::new();
        println!("Please enter your age: ");
    io::stdin().read_line(&mut input).expect("Not a valid string");
    let age:u8 = input.trim().parse().expect("Not a valid number");
    if age <= 27
    {
        println!("Your annual incentive is N1,300,000");
    }
    else if age >= 28 && age <= 39
    {
        println!("Your annual incentive is N1,480,000");
    }

    else if age >= 40
    {
        println!("Your annual incentive is N1,560,000");
    }
}
    
}
