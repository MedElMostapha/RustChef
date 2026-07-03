use std::collections::HashMap;

pub fn shannon(input: &str) -> f64 {
    let bytes = input.as_bytes();
    let len = bytes.len() as f64;
    if len == 0.0 {
        return 0.0;
    }

    let mut freq: HashMap<u8, usize> = HashMap::new();
    for &b in bytes {
        *freq.entry(b).or_insert(0) += 1;
    }

    let entropy: f64 = freq
        .values()
        .map(|&count| {
            let p = count as f64 / len;
            -p * p.log2()
        })
        .sum();

    (entropy * 1000.0).round() / 1000.0
}
