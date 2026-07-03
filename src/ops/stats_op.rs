use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct TextStats {
    pub char_count: usize,
    pub word_count: usize,
    pub line_count: usize,
    pub byte_count: usize,
    pub letter_count: usize,
    pub digit_count: usize,
    pub whitespace_count: usize,
    pub punctuation_count: usize,
}

pub fn compute(input: &str) -> TextStats {
    let char_count = input.chars().count();
    let byte_count = input.len();
    let line_count = if input.is_empty() {
        0
    } else {
        input.lines().count()
    };
    let word_count = input
        .split_whitespace()
        .filter(|w| !w.is_empty())
        .count();

    let mut letter_count = 0;
    let mut digit_count = 0;
    let mut whitespace_count = 0;
    let mut punctuation_count = 0;

    for c in input.chars() {
        if c.is_alphabetic() {
            letter_count += 1;
        } else if c.is_ascii_digit() {
            digit_count += 1;
        } else if c.is_whitespace() {
            whitespace_count += 1;
        } else if c.is_ascii_punctuation() {
            punctuation_count += 1;
        }
    }

    TextStats {
        char_count,
        word_count,
        line_count,
        byte_count,
        letter_count,
        digit_count,
        whitespace_count,
        punctuation_count,
    }
}

#[allow(dead_code)]
pub fn freq_table(input: &str) -> HashMap<char, usize> {
    let mut freq: HashMap<char, usize> = HashMap::new();
    for c in input.chars() {
        *freq.entry(c).or_insert(0) += 1;
    }
    freq
}
