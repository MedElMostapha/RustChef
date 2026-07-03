use anyhow::Result;
use percent_encoding::{percent_decode, percent_encode, NON_ALPHANUMERIC};

pub fn encode(input: &str) -> String {
    percent_encode(input.as_bytes(), NON_ALPHANUMERIC).to_string()
}

pub fn decode(input: &str) -> Result<String> {
    let bytes = percent_decode(input.as_bytes())
        .decode_utf8()
        .map_err(|e| anyhow::anyhow!("Invalid URL encoding: {}", e))?;
    Ok(bytes.to_string())
}
