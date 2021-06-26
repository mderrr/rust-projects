use std::io;
use std::io::Write; // <--- bring flush() into scope

fn print_vector(vector: &Vec<char>) {
    for character in vector.iter() {
        print!("{}", character);
    }
}

fn convert_to_binary(string: &String) -> Vec<char> {
    let mut string_bytes = string.clone().into_bytes();
    let mut vector: Vec<char> = Vec::new();

    if string.ends_with('\n') {
        string_bytes.remove( string_bytes.len() - 1 ); // Remove the Newline character
    }

    for character_byte in string_bytes {
        let character_bits = format!("{:b}", character_byte);

        for character_bit in character_bits.chars() {
            vector.push(character_bit);
        }
    }

    vector
}

fn bitwise_right_shift(char_vector: &Vec<char>) -> Vec<char> {
    let mut shifted_vector: Vec<char> = vec![ '0'; char_vector.len() ];

    shifted_vector.Vec::from_iter(data[1..4].iter().cloned())

    shifted_vector
}

fn main() {
    let mut guess = String::new();

    println!("Enter your input");
    
    print!(" > ");
    io::stdout().flush().unwrap(); // Flush to print immedialty


    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");


    let binary_guess: Vec<char> = convert_to_binary(&guess);

    print!("Guess in binary is ");
    print_vector(&binary_guess);
    println!();

    let shifted_binary_guess: Vec<char> = bitwise_right_shift(&binary_guess);

     print!("Guess in binary is ");
    print_vector(&shifted_binary_guess);
    println!();
}
