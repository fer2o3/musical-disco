use musical_disco::{bytes_to_hex, hex_to_bytes, xor};

fn main() {
    let hex_a = "1c0111001f010100061a024b53535009181c";
    let hex_b = "686974207468652062756c6c277320657965";
    let expected = "746865206b696420646f6e277420706c6179";

    let bytes_a = hex_to_bytes(hex_a).expect("Invalid hex A");
    let bytes_b = hex_to_bytes(hex_b).expect("Invalid hex B");

    let result = xor(&bytes_a, &bytes_b);

    let result_hex = bytes_to_hex(&result);

    println!("Challenge 2: Fixed XOR");
    println!("Input A:      {}", hex_a);
    println!("Input B:      {}", hex_b);
    println!("XOR Result:   {}", result_hex);
    println!("Expected:     {}", expected);
    println!(
        "✓ Success: {}",
        if result_hex == expected {
            "PASSED"
        } else {
            "FAILED"
        }
    );
}
