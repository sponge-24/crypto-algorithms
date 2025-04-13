use std::io::{self, Write};

fn myszkowski_encrypt(plaintext: &str, keyword: &str) -> String {
    let mut grid = vec![];
    let num_cols = keyword.len();
    let num_rows = (plaintext.len() + num_cols - 1) / num_cols;

    // Fill the grid with plaintext characters and padding
    let mut padded_plaintext = plaintext.to_string();
    while padded_plaintext.len() < num_cols * num_rows {
        padded_plaintext.push('.');
    }

    // Create the grid
    for i in 0..num_rows {
        let start = i * num_cols;
        let end = start + num_cols;
        let row: Vec<char> = padded_plaintext[start..end].chars().collect();
        grid.push(row);
    }

    // Sort the keyword and get the column order
    let mut keyword_chars: Vec<(char, usize)> = keyword
        .chars()
        .enumerate()
        .map(|(index, char)| (char, index)) // Swap tuple to (char, index)
        .collect();
    keyword_chars.sort_by(|a, b| a.0.cmp(&b.0));

    // Rearrange the columns based on the sorted keyword
    let mut column_order = vec![];
    for (_, index) in keyword_chars {
        column_order.push(index);
    }

    // Read columns in the new order to form the ciphertext
    let mut ciphertext = String::new();
    for col in column_order {
        for row in 0..num_rows {
            ciphertext.push(grid[row][col]);
        }
    }

    ciphertext
}

fn myszkowski_decrypt(ciphertext: &str, keyword: &str) -> String {
    let num_cols = keyword.len();
    let num_rows = (ciphertext.len() + num_cols - 1) / num_cols;

    // Create an empty grid
    let mut grid = vec![vec!['.'; num_cols]; num_rows];
    let mut column_order = vec![];

    // Sort the keyword and get the column order
    let mut keyword_chars: Vec<(char, usize)> = keyword
        .chars()
        .enumerate()
        .map(|(index, char)| (char, index)) // Swap tuple to (char, index)
        .collect();
    keyword_chars.sort_by(|a, b| a.0.cmp(&b.0));

    for (_, index) in keyword_chars {
        column_order.push(index);
    }

    // Fill the grid with ciphertext in the new column order
    let mut index = 0;
    for col in column_order {
        for row in 0..num_rows {
            grid[row][col] = ciphertext.chars().nth(index).unwrap();
            index += 1;
        }
    }

    // Read the grid row by row to form the decrypted plaintext
    let mut decrypted = String::new();
    for row in 0..num_rows {
        for col in 0..num_cols {
            decrypted.push(grid[row][col]);
        }
    }

    decrypted.trim_end_matches('.').to_string() // Remove padding
}

fn main() {
    let mut input = String::new();
    let mut key_input = String::new();

    print!("Enter plaintext: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let plaintext = input.trim();

    print!("Enter keyword: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut key_input).unwrap();
    let keyword = key_input.trim();

    let encrypted = myszkowski_encrypt(plaintext, keyword);
    let decrypted = myszkowski_decrypt(&encrypted, keyword);

    println!("\nEncrypted: {}", encrypted);
    println!("Decrypted: {}", decrypted);
}
