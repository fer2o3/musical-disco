use musical_disco::{hex_to_bytes, xor};

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

fn main() {
    let input =
        hex_to_bytes("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736")
            .unwrap();

    let (key, plaintext) = (0u8..=255)
        .map(|k| (k, xor(&input, &[k])))
        .max_by_key(|(_k, bytes)| bytes.iter().map(|&b| frequency_score(b)).sum::<u64>())
        .unwrap();

    println!("Challenge 2: Single-byte XOR cipher");
    println!("Found key:           0x{:02x}", key);
    println!(
        "Decrypted message:     {}",
        String::from_utf8_lossy(&plaintext)
    );
}
