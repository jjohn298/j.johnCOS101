use std::io;

fn main() {
    // Outputting the food menu
    println!("Welcome to The  Yebo Restaurant!\nWhat would you like to eat today?");
    println!("Would you like to see the menu?");

    let mut response = String::new();
    io::stdin().read_line(&mut response).expect("Failed to read input");
    let answer = response.trim().to_uppercase();
     if answer == "Y" {



    println!("P - Poundo Yam / Edinkaiko Soup ---------------- N3,200");
    println!("F - Fried Rice & Chicken        ---------------- N3,000");
    println!("A - Amala & Ewedu Soup          ---------------- N2,500");
    println!("E - Eba & Egusi Soup            ---------------- N2,000");
    println!("W - White Rice & Stew           ---------------- N2,500");

    //Outputting the question for user to pick a meal option

    println!("\nEnter food choice letter (P, F, A, E, W):");

    let mut food_input = String::new();
    io::stdin().read_line(&mut food_input).expect("Failed to read input");
    let food_choice = food_input.trim().to_uppercase();

    // Ask for how many portions

    println!("Enter quantity:");

    let mut quantity_ = String::new();
    io::stdin().read_line(&mut quantity_).expect("Failed to read input");
    
    // Convert the quantity into a number for calculations

    let quantity: f32 = quantity_.trim().parse().expect("Please enter a valid number");

    let mut price: f32 = 0.0;

    // Asssigning the letter to its set price

    if food_choice == "P" {

        price = 3_200.0;
    } else if food_choice == "F" 
    {
        price = 3_000.0;
    } 
    else if food_choice == "A" 
    {
        price = 2_500.0;
    }
     else if food_choice == "E" 
     {
        price = 2_000.0;
    }
     else if food_choice == "W" 
     {
        price = 2_500.0;
    }
     else {

        println!("Please choose a valid food on the menu.");

        return;
    }

    // Work out the subtotal

    let mut total: f32 = price * quantity;

    println!("\nInitial Charge: N{}", total);

    // Apply discount if the bill is above 10,000

    if total > 10_000.0 {

        let discount: f32 = total * 0.05;
        total = total - discount;

        println!("5% Discount applied: -N{}", discount);
    }

    println!("Final Charge: N{}", total);
    println!("Thanks for visiting Yebo Restaurant");
}
else {
    println!("Then please take our free bottle of water.\nThank you for visiting our restaurant");
    return;
}
}
