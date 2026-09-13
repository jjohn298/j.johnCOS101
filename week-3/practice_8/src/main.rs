fn main() {
    let mut fees = 25_000;  // Fixed: Added 'mut' to allow this variable to change
    println!("fees is {} ", fees); // Fixed: changed printin! to println!

    fees = 35_000;
    println!("fees changed is {}", fees); // Fixed: changed printin! to println!
}
