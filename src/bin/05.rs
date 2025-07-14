use musical_disco::{bytes_to_hex, xor};

fn main() {
    let plaintext = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
    let key = "ICE";

    let plaintext_bytes = plaintext.as_bytes();
    let key_bytes = key.as_bytes();

    let ciphertext = xor(plaintext_bytes, key_bytes);

    let hex_result = bytes_to_hex(&ciphertext);

    println!("Challenge 5: Implement repeating-key XOR");
    println!("Plaintext: {}", plaintext);
    println!("Key: {}", key);
    println!("Ciphertext (hex): {}", hex_result);

    let expected = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";

    if hex_result == expected {
        println!("Result matches expected output!");
    } else {
        println!("Result does not match expected output");
        println!("Expected: {}", expected);
    }
}
