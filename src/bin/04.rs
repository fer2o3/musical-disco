use musical_disco::{hex_to_bytes, xor};
use std::fs;

fn frequency_score(b: u8) -> u64 {
    match (b as char).to_ascii_uppercase() {
        ' ' => 20000,
        'E' => 12000,
        'T' => 9000,
        'A' | 'I' | 'N' | 'O' | 'S' => 8000,
        'H' => 6400,
        'R' => 6200,
        'D' => 4400,
        'L' => 4000,
        'U' => 3400,
        'C' | 'M' => 3000,
        'F' => 2500,
        'W' | 'Y' => 2000,
        'G' | 'P' => 1700,
        'B' => 1600,
        'V' => 1200,
        'K' => 800,
        'Q' => 500,
        'J' | 'X' => 400,
        'Z' => 200,
        _ => 0,
    }
}

fn find_best_single_xor(ciphertext: &[u8]) -> Option<(u8, Vec<u8>, u64)> {
    (0u8..=255)
        .map(|key| {
            let plaintext = xor(ciphertext, &[key]);
            let score = plaintext.iter().map(|&b| frequency_score(b)).sum::<u64>();
            (key, plaintext, score)
        })
        .max_by_key(|(_, _, score)| *score)
}

fn main() {
    let file_content =
        fs::read_to_string("inputs/04.in").expect("Failed to read file inputs/04.in");

    let mut best_overall: Option<(u8, Vec<u8>, u64)> = None;

    for line in file_content.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        if let Ok(ciphertext) = hex_to_bytes(line) {
            if let Some((key, plaintext, score)) = find_best_single_xor(&ciphertext) {
                if best_overall
                    .as_ref()
                    .map_or(true, |(_, _, best_score)| score > *best_score)
                {
                    best_overall = Some((key, plaintext, score));
                }
            }
        }
    }

    match best_overall {
        Some((key, plaintext, score)) => {
            println!("Challenge 4: Detect single-character XOR");
            println!("Found key:             0x{:02x}", key);
            println!("Score:                 {}", score);
            println!(
                "Decrypted message:     {}",
                String::from_utf8_lossy(&plaintext)
            );
        }
        None => {
            println!("No valid decryption found!");
        }
    }
}
