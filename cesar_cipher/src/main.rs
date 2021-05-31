use rand::Rng;

//def encrypt(string, key):
//    cipher = ""

//    for character in string: 
//        if (character == " "):
//            cipher += character
//
//        elif (character.isupper()):
//            cipher += chr((ord(character) + key - 65) % 26 + 65)
//
//        else:
//            cipher += chr((ord(character) + key - 97) % 26 + 97)
//    
//    return cipher

fn encrypt(message: &str, decryption_key: u32) {
    let mut encrypted_message = String::new();
    const message_length: usize = message.chars().count();


    let mut encrypted_message_array: [char; message_length]; 

    for (i, letter) in message.chars().enumerate() {
        if letter == ' ' {
            println!("EMPY");
        }
        encrypted_message_array[i] = letter;
        println!("AAAAAAA {}", letter);
    }

    println!("BBBBBBBBBBB {}", encrypted_message_array);

}

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..101);

    encrypt("asadf wwdw", 43)
    //println!("Hello, world {}!", secret_number);
}
