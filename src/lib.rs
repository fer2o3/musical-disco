fn hex_char_to_nibble(c: char) -> Result<u8, &'static str> {
    match c {
        '0'..='9' => Ok(c as u8 - b'0'),
        'a'..='f' => Ok(c as u8 - b'a' + 10),
        'A'..='F' => Ok(c as u8 - b'A' + 10),
        _ => Err("Invalid hex character"),
    }
}

pub fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, &'static str> {
    if hex.len() % 2 != 0 {
        return Err("Hex string must have even length");
    }

    let mut bytes = Vec::new();
    let chars: Vec<char> = hex.chars().collect();

    for chunk in chars.chunks(2) {
        let high = hex_char_to_nibble(chunk[0])?;
        let low = hex_char_to_nibble(chunk[1])?;
        bytes.push((high << 4) | low);
    }

    Ok(bytes)
}

pub fn bytes_to_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

pub fn bytes_to_base64(bytes: &[u8]) -> String {
    const BASE64_CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    let mut result = String::new();

    for chunk in bytes.chunks(3) {
        let mut buf = [0u8; 3];
        for (i, &byte) in chunk.iter().enumerate() {
            buf[i] = byte;
        }

        let b1 = buf[0] >> 2;
        let b2 = ((buf[0] & 0x03) << 4) | (buf[1] >> 4);
        let b3 = ((buf[1] & 0x0f) << 2) | (buf[2] >> 6);
        let b4 = buf[2] & 0x3f;

        result.push(BASE64_CHARS[b1 as usize] as char);
        result.push(BASE64_CHARS[b2 as usize] as char);

        if chunk.len() > 1 {
            result.push(BASE64_CHARS[b3 as usize] as char);
        } else {
            result.push('=');
        }

        if chunk.len() > 2 {
            result.push(BASE64_CHARS[b4 as usize] as char);
        } else {
            result.push('=');
        }
    }

    result
}

pub fn hex_to_base64(hex: &str) -> Result<String, &'static str> {
    let bytes = hex_to_bytes(hex)?;
    Ok(bytes_to_base64(&bytes))
}

pub fn xor(a: &[u8], b: &[u8]) -> Result<Vec<u8>, &'static str> {
    if a.len() != b.len() {
        return Err("Buffers must be equal length");
    }

    Ok(a.iter().zip(b.iter()).map(|(x, y)| x ^ y).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_to_bytes() {
        assert_eq!(hex_to_bytes("48656c6c6f").unwrap(), b"Hello");
        assert_eq!(hex_to_bytes("00").unwrap(), vec![0]);
        assert_eq!(hex_to_bytes("ff").unwrap(), vec![255]);
        assert!(hex_to_bytes("xyz").is_err());
        assert!(hex_to_bytes("1").is_err());
    }

    #[test]
    fn test_bytes_to_hex() {
        assert_eq!(bytes_to_hex(b"Hello"), "48656c6c6f");
        assert_eq!(bytes_to_hex(&[0, 255, 16]), "00ff10");
        assert_eq!(bytes_to_hex(&[]), "");
    }

    #[test]
    fn test_bytes_to_base64() {
        assert_eq!(bytes_to_base64(b"Hello"), "SGVsbG8=");
        assert_eq!(bytes_to_base64(b"Hi"), "SGk=");
        assert_eq!(bytes_to_base64(b"H"), "SA==");
        assert_eq!(bytes_to_base64(b""), "");
    }

    #[test]
    fn test_xor() {
        assert_eq!(xor(&[1, 2, 3], &[4, 5, 6]).unwrap(), vec![5, 7, 5]);
        assert_eq!(xor(&[0xff], &[0x00]).unwrap(), vec![0xff]);
        assert_eq!(xor(&[0xaa], &[0xaa]).unwrap(), vec![0x00]);
        assert!(xor(&[1, 2], &[1]).is_err());
    }
}
