# RustChef

A CyberChef-inspired command-line tool for data transformations, encoding, decoding, hashing, extraction, and analysis.

## Operations

| Operation | Description |
|-----------|-------------|
| `base64-encode` | Encode data to Base64 |
| `base64-decode` | Decode Base64 data |
| `hex-encode` | Encode data to hexadecimal |
| `hex-decode` | Decode hexadecimal data |
| `url-encode` | URL-percent-encode a string |
| `url-decode` | URL-percent-decode a string |
| `rot13` | Apply ROT13 transformation |
| `xor` | XOR data with a repeating key |
| `md5` | Compute MD5 hash |
| `sha1` | Compute SHA1 hash |
| `sha256` | Compute SHA256 hash |
| `extract-ips` | Extract IPv4 and IPv6 addresses |
| `extract-urls` | Extract HTTP/HTTPS/FTP URLs |
| `extract-emails` | Extract email addresses |
| `stats` | Compute text statistics |
| `entropy` | Compute Shannon entropy |

## Installation

### Using Cargo

```bash
cargo install --path .
```

### Using Docker

```bash
docker build -t rustchef .
```

## Usage

Input can be provided as a command-line argument or via stdin (pipe).

### Encoding / Decoding

```bash
# Base64
rustchef base64-encode "hello world"
rustchef base64-decode "aGVsbG8gd29ybGQ="

# Hex
rustchef hex-encode "hello"
rustchef hex-decode "68656c6c6f"

# URL
rustchef url-encode "hello world"
rustchef url-decode "hello%20world"
```

### Hashing

```bash
rustchef md5 "hello"
rustchef sha1 "hello"
rustchef sha256 "hello"
```

### Transformations

```bash
# ROT13
rustchef rot13 "Hello, World!"

# XOR with key
rustchef xor --key mykey "secret data"
```

### Extraction

```bash
rustchef extract-ips "Server: 192.168.1.1, gateway: 10.0.0.1"
rustchef extract-urls "Visit https://example.com today"
rustchef extract-emails "Contact: user@example.com"
```

### Analysis

```bash
rustchef stats "Hello\nWorld"
rustchef entropy "aaaa"
```

### Using stdin

```bash
echo "hello" | rustchef base64-encode
cat file.txt | rustchef extract-emails
```

## Examples

See the `samples/` directory for sample input files and expected outputs.

```bash
# Process sample text
rustchef stats samples/input.txt

# Pipeline operations
cat samples/input.txt | rustchef extract-emails | rustchef base64-encode
```

## Testing

```bash
cargo test
```

## Code Quality

```bash
cargo clippy
```

## Limitations

- Binary output from decode/XOR operations is displayed as hex when not valid UTF-8
- The tool processes data as text; binary file operations are limited
- Extraction regexes may not catch all edge cases (e.g., unusual URL formats)
- Only supports single-operation invocations (no recipe chaining within one call)
