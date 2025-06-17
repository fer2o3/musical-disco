use musical_disco::hex_to_base64;

fn main() {
    let hex = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let expected = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

    match hex_to_base64(hex) {
        Ok(result) => {
            println!("Challenge 1: Convert hex to base64");
            println!("Input (hex):  {}", hex);
            println!("Output:       {}", result);
            println!("Expected:     {}", expected);
            println!(
                "✓ Success: {}",
                if result == expected {
                    "PASSED"
                } else {
                    "FAILED"
                }
            );
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
