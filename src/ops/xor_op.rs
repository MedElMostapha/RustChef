use anyhow::{bail, Result};

pub fn apply(input: &str, key: &str) -> Result<String> {
    if key.is_empty() {
        bail!("XOR key cannot be empty");
    }
    let key_bytes = key.as_bytes();
    let input_bytes = input.as_bytes();
    let result: Vec<u8> = input_bytes
        .iter()
        .enumerate()
        .map(|(i, b)| b ^ key_bytes[i % key_bytes.len()])
        .collect();
    match String::from_utf8(result.clone()) {
        Ok(s) => Ok(s),
        Err(_) => {
            let hex: String = result.iter().map(|b| format!("{:02x}", b)).collect();
            Ok(format!("<binary data, hex: {}>", hex))
        }
    }
}
