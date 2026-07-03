use anyhow::{bail, Result};

pub fn encode(input: &str) -> String {
    input
        .as_bytes()
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect()
}

pub fn decode(input: &str) -> Result<String> {
    let compact: String = input.chars().filter(|c| !c.is_whitespace()).collect();
    if !compact.len().is_multiple_of(2) {
        bail!("Hex string must have an even number of characters");
    }
    let bytes = (0..compact.len())
        .step_by(2)
        .map(|i| {
            u8::from_str_radix(&compact[i..i + 2], 16)
                .map_err(|e| anyhow::anyhow!("Invalid hex at position {}: {}", i, e))
        })
        .collect::<Result<Vec<u8>>>()?;
    match String::from_utf8(bytes.clone()) {
        Ok(s) => Ok(s),
        Err(_) => {
            let hex: String = bytes.iter().map(|b| format!("{:02x}", b)).collect();
            Ok(format!("<binary data, hex: {}>", hex))
        }
    }
}
