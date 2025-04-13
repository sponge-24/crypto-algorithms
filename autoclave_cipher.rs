use std::io::{self, Write};

fn autokey_encrypt(plaintext: &str, key: &str) -> String {
    let mut full_key = key.to_uppercase();
    let filtered_plaintext: String = plaintext.chars().filter(|c| c.is_ascii_alphabetic()).collect();
    full_key.push_str(&filtered_plaintext.to_uppercase());

    let key_bytes = full_key.as_bytes();
    let mut key_index = 0;

    plaintext
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let is_upper = c.is_ascii_uppercase();
                let base = if is_upper { b'A' } else { b'a' };
                let p = c.to_ascii_uppercase() as u8 - b'A';
                let k = key_bytes[key_index] - b'A';
                key_index += 1;
                ((p + k) % 26 + base) as char
            } else {
                c
            }
        })
        .collect()
}

fn autokey_decrypt(ciphertext: &str, key: &str) -> String {
    let mut key_stream = key.to_uppercase();
    let mut result = String::new();

    for c in ciphertext.chars() {
        if c.is_ascii_alphabetic() {
            let is_upper = c.is_ascii_uppercase();
            let base = if is_upper { b'A' } else { b'a' };

            let c_index = c.to_ascii_uppercase() as u8 - b'A';
            let k = key_stream.as_bytes()[result.len()] - b'A';
            let p = ((26 + c_index - k) % 26 + base) as char;
            result.push(p);

            // extend key stream with recovered plaintext
            key_stream.push(p.to_ascii_uppercase());
        } else {
            result.push(c);
        }
    }

    result
}

fn main() {
    let mut input = String::new();
    let mut key_input = String::new();

    print!("Enter plaintext: ");
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

    let encrypted = autokey_encrypt(text, key);
    let decrypted = autokey_decrypt(&encrypted, key);

    println!("\nEncrypted: {}", encrypted);
    println!("Decrypted: {}", decrypted);
}
