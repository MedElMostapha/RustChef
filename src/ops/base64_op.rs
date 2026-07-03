use anyhow::Result;

pub fn encode(input: &str) -> String {
    use base64::Engine;
    base64::engine::general_purpose::STANDARD.encode(input.as_bytes())
}

pub fn decode(input: &str) -> Result<String> {
    use base64::Engine;
    let bytes = base64::engine::general_purpose::STANDARD
        .decode(input.trim())
        .map_err(|e| anyhow::anyhow!("Invalid Base64 input: {}", e))?;
    match String::from_utf8(bytes.clone()) {
        Ok(s) => Ok(s),
        Err(_) => {
            let hex: String = bytes.iter().map(|b| format!("{:02x}", b)).collect();
            Ok(format!("<binary data, hex: {}>", hex))
        }
    }
}
