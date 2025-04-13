use std::io::{self, Write};

fn route_cipher_encrypt(plaintext: &str, rows: usize, cols: usize) -> String {
    let mut grid = vec![vec![' '; cols]; rows];

    // Fill the grid row by row
    let mut idx = 0;
    for r in 0..rows {
        for c in 0..cols {
            if idx < plaintext.len() {
                grid[r][c] = plaintext.chars().nth(idx).unwrap();
                idx += 1;
            } else {
                grid[r][c] = '.'; // Padding character
            }
        }
    }

    // Read the ciphertext column by column
    let mut ciphertext = String::new();
    for c in 0..cols {
        for r in 0..rows {
            ciphertext.push(grid[r][c]);
        }
    }

    ciphertext
}

fn route_cipher_decrypt(ciphertext: &str, rows: usize, cols: usize) -> String {
    let mut grid = vec![vec![' '; cols]; rows];

    // Fill the grid column by column
    let mut idx = 0;
    for c in 0..cols {
        for r in 0..rows {
            if idx < ciphertext.len() {
                grid[r][c] = ciphertext.chars().nth(idx).unwrap();
                idx += 1;
            }
        }
    }

    // Read the plaintext row by row
    let mut plaintext = String::new();
    for r in 0..rows {
        for c in 0..cols {
            plaintext.push(grid[r][c]);
        }
    }

    plaintext
}

fn main() {
    let mut input = String::new();
    let mut rows_input = String::new();
    let mut cols_input = String::new();

    print!("Enter plaintext: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let plaintext = input.trim();

    print!("Enter number of rows: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut rows_input).unwrap();
    let rows: usize = rows_input.trim().parse().unwrap();

    print!("Enter number of columns: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut cols_input).unwrap();
    let cols: usize = cols_input.trim().parse().unwrap();

    let encrypted = route_cipher_encrypt(plaintext, rows, cols);
    let decrypted = route_cipher_decrypt(&encrypted, rows, cols);

    println!("\nEncrypted: {}", encrypted);
    println!("Decrypted: {}", decrypted);
}
