use std::io;

fn main() {
    let v = vec!['C', 'O', 'M', 'P', 'U', 'T', 'E', 'R'];

    let mut input1 = String::new();

    println!("Enter an index value btw (0 - 7)");
    
    // Corrected "read_Line" to lowercase "read_line"
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    
    let index: usize = input1.trim().parse().expect("Invalid input");

    // Rust indexing returns a reference (&char), so we use * to dereference it into a char
    let ch: char = *v[index];

    print!("{} is the character for index [{}]\n", ch, index);
}
