use std::io::{self, Write};

fn beaufort_cipher(input: &str, key: &str) -> String {
    let upper_key = key.to_uppercase();
    let key_bytes = upper_key.as_bytes();
    let mut key_index = 0;

    input.chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let is_upper = c.is_ascii_uppercase();
                let base = if is_upper { b'A' } else { b'a' };

                let p = c.to_ascii_uppercase() as u8 - b'A'; // plaintext letter index
                let k = key_bytes[key_index % key_bytes.len()] - b'A'; // key letter index

                key_index += 1;
                ((k + 26 - p) % 26 + base) as char // (K - P) % 26
            } else {
                c
            }
        })
        .collect()
}

fn main() {
    let mut input = String::new();
    let mut key_input = String::new();

    print!("Enter text: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let text = input.trim();

    print!("Enter keyword: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut key_input).unwrap();
    let key = key_input.trim();

    if key.is_empty() || !key.chars().all(|c| c.is_ascii_alphabetic()) {
        println!("Invalid key. It must be a non-empty alphabetic string.");
        return;
    }

    let ciphered = beaufort_cipher(text, key);

    println!("\nCipher Text (Beaufort): {}", ciphered);
    println!("Original (deciphered using same function): {}", beaufort_cipher(&ciphered, key));
}
