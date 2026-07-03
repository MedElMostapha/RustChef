use regex::Regex;

pub fn extract_ips(input: &str) -> Vec<String> {
    let ipv4_re = Regex::new(
        r"\b(?:(?:25[0-5]|2[0-4]\d|[01]?\d\d?)\.){3}(?:25[0-5]|2[0-4]\d|[01]?\d\d?)\b",
    )
    .unwrap();
    let ipv6_re = Regex::new(
        r"\b(?:[0-9a-fA-F]{1,4}:){7}[0-9a-fA-F]{1,4}\b",
    )
    .unwrap();

    let mut ips: Vec<String> = Vec::new();
    for cap in ipv4_re.find_iter(input) {
        ips.push(cap.as_str().to_string());
    }
    for cap in ipv6_re.find_iter(input) {
        ips.push(cap.as_str().to_string());
    }
    ips.sort();
    ips.dedup();
    ips
}

pub fn extract_urls(input: &str) -> Vec<String> {
    let re = Regex::new(r#"https?://[^\s<>"']+|ftp://[^\s<>"']+"#).unwrap();
    let mut urls: Vec<String> = re
        .find_iter(input)
        .map(|m| m.as_str().to_string())
        .collect();
    urls.sort();
    urls.dedup();
    urls
}

pub fn extract_emails(input: &str) -> Vec<String> {
    let re = Regex::new(r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}\b").unwrap();
    let mut emails: Vec<String> = re
        .find_iter(input)
        .map(|m| m.as_str().to_string())
        .collect();
    emails.sort();
    emails.dedup();
    emails
}
