use std::process::Command;

fn run(args: &[&str], input: Option<&str>) -> Result<String, String> {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_rustchef"));
    cmd.args(args);

    if let Some(data) = input {
        cmd.arg(data);
    }

    let output = cmd.output().expect("Failed to execute command");
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    // Remove exactly one trailing newline added by println!
    let trimmed = stdout.strip_suffix('\n').unwrap_or(&stdout).to_string();
    if output.status.success() {
        Ok(trimmed)
    } else {
        Err(String::from_utf8_lossy(&output.stderr).trim().to_string())
    }
}

fn run_stdin(args: &[&str], stdin: &str) -> Result<String, String> {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_rustchef"));
    cmd.args(args);
    cmd.stdin(std::process::Stdio::piped());
    cmd.stdout(std::process::Stdio::piped());

    let mut child = cmd.spawn().expect("Failed to spawn command");
    use std::io::Write;
    child
        .stdin
        .take()
        .unwrap()
        .write_all(stdin.as_bytes())
        .unwrap();
    let output = child.wait_with_output().expect("Failed to wait");
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let trimmed = stdout.strip_suffix('\n').unwrap_or(&stdout).to_string();
    if output.status.success() {
        Ok(trimmed)
    } else {
        Err(String::from_utf8_lossy(&output.stderr).trim().to_string())
    }
}

#[test]
fn test_base64_encode() {
    let result = run(&["base64-encode"], Some("hello")).unwrap();
    assert_eq!(result, "aGVsbG8=");
}

#[test]
fn test_base64_decode() {
    let result = run(&["base64-decode"], Some("aGVsbG8=")).unwrap();
    assert_eq!(result, "hello");
}

#[test]
fn test_base64_roundtrip() {
    let input = "Hello, Rust! 123 🦀";
    let encoded = run(&["base64-encode"], Some(input)).unwrap();
    let decoded = run(&["base64-decode"], Some(&encoded)).unwrap();
    assert_eq!(decoded, input);
}

#[test]
fn test_hex_encode() {
    let result = run(&["hex-encode"], Some("hello")).unwrap();
    assert_eq!(result, "68656c6c6f");
}

#[test]
fn test_hex_decode() {
    let result = run(&["hex-decode"], Some("68656c6c6f")).unwrap();
    assert_eq!(result, "hello");
}

#[test]
fn test_hex_roundtrip() {
    let input = "Hex test 123!";
    let encoded = run(&["hex-encode"], Some(input)).unwrap();
    let decoded = run(&["hex-decode"], Some(&encoded)).unwrap();
    assert_eq!(decoded, input);
}

#[test]
fn test_url_encode() {
    let result = run(&["url-encode"], Some("hello world")).unwrap();
    assert_eq!(result, "hello%20world");
}

#[test]
fn test_url_decode() {
    let result = run(&["url-decode"], Some("hello%20world")).unwrap();
    assert_eq!(result, "hello world");
}

#[test]
fn test_url_roundtrip() {
    let input = "a b c d e f+g h&i=j?k/l";
    let encoded = run(&["url-encode"], Some(input)).unwrap();
    let decoded = run(&["url-decode"], Some(&encoded)).unwrap();
    assert_eq!(decoded, input);
}

#[test]
fn test_rot13() {
    let result = run(&["rot13"], Some("Hello, World!")).unwrap();
    assert_eq!(result, "Uryyb, Jbeyq!");
}

#[test]
fn test_rot13_roundtrip() {
    let input = "Rust is awesome!";
    let once = run(&["rot13"], Some(input)).unwrap();
    let twice = run(&["rot13"], Some(&once)).unwrap();
    assert_eq!(twice, input);
}

#[test]
fn test_rot13_numbers() {
    let result = run(&["rot13"], Some("12345")).unwrap();
    assert_eq!(result, "12345");
}

#[test]
fn test_xor() {
    // "hello" XOR "key" repeated: h^k, e^e, l^y, l^k, o^e
    // 104^107=3, 101^101=0, 108^121=21, 108^107=7, 111^101=10
    let result = run(&["xor", "--key", "key"], Some("hello")).unwrap();
    assert_eq!(result, "\x03\x00\x15\x07\x0a");
}

#[test]
fn test_xor_roundtrip() {
    let input = "secret message";
    let key = "xyz";
    let encrypted = run(&["xor", "--key", key], Some(input)).unwrap();
    let decrypted = run(&["xor", "--key", key], Some(&encrypted)).unwrap();
    assert_eq!(decrypted, input);
}

#[test]
fn test_md5() {
    let result = run(&["md5"], Some("hello")).unwrap();
    assert_eq!(result, "5d41402abc4b2a76b9719d911017c592");
}

#[test]
fn test_sha1() {
    let result = run(&["sha1"], Some("hello")).unwrap();
    assert_eq!(result, "aaf4c61ddcc5e8a2dabede0f3b482cd9aea9434d");
}

#[test]
fn test_sha256() {
    let result = run(&["sha256"], Some("hello")).unwrap();
    assert_eq!(
        result,
        "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
    );
}

#[test]
fn test_entropy() {
    let result = run(&["entropy"], Some("aaaa")).unwrap();
    let val: f64 = result.parse().unwrap();
    assert!(val < 0.01);
}

#[test]
fn test_entropy_high() {
    let result = run(&["entropy"], Some("abcd1234!@#$")).unwrap();
    let val: f64 = result.parse().unwrap();
    assert!(val > 3.0);
}

#[test]
fn test_stats() {
    let result = run(&["stats"], Some("Hello\nWorld! 123")).unwrap();
    assert!(result.contains("Character count:  16"));
    assert!(result.contains("Word count:       3"));
    assert!(result.contains("Line count:       2"));
    assert!(result.contains("Byte count:       16"));
}

#[test]
fn test_empty_stats() {
    let result = run(&["stats"], Some("")).unwrap();
    assert!(result.contains("Character count:  0"));
    assert!(result.contains("Word count:       0"));
    assert!(result.contains("Line count:       0"));
    assert!(result.contains("Byte count:       0"));
}

#[test]
fn test_extract_ips() {
    let input = "Server IPs: 192.168.1.1, 10.0.0.1, and invalid 999.999.999.999";
    let result = run(&["extract-ips"], Some(input)).unwrap();
    assert!(result.contains("192.168.1.1"));
    assert!(result.contains("10.0.0.1"));
    assert!(!result.contains("999.999.999.999"));
}

#[test]
fn test_extract_urls() {
    let input = "Visit https://example.com and http://test.org/path!";
    let result = run(&["extract-urls"], Some(input)).unwrap();
    assert!(result.contains("https://example.com"));
    assert!(result.contains("http://test.org/path"));
}

#[test]
fn test_extract_emails() {
    let input = "Contact: user@example.com, admin@test.org";
    let result = run(&["extract-emails"], Some(input)).unwrap();
    assert!(result.contains("user@example.com"));
    assert!(result.contains("admin@test.org"));
}

#[test]
fn test_stdin_input() {
    let result = run_stdin(&["base64-encode"], "stdin input").unwrap();
    assert_eq!(result, "c3RkaW4gaW5wdXQ=");
}

#[test]
fn test_hex_decode_invalid() {
    let result = run(&["hex-decode"], Some("xyz"));
    assert!(result.is_err());
}

#[test]
fn test_hex_decode_odd_length() {
    let result = run(&["hex-decode"], Some("abc"));
    assert!(result.is_err());
}

#[test]
fn test_base64_decode_invalid() {
    let result = run(&["base64-decode"], Some("!!!invalid!!!"));
    assert!(result.is_err());
}

#[test]
fn test_xor_empty_key() {
    let result = run(&["xor", "--key", ""], Some("test"));
    assert!(result.is_err());
}

#[test]
fn test_empty_stats_again() {
    let result = run(&["stats"], Some("")).unwrap();
    assert!(result.contains("Character count:  0"));
    assert!(result.contains("Word count:       0"));
    assert!(result.contains("Line count:       0"));
    assert!(result.contains("Byte count:       0"));
}
