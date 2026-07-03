mod cli;
mod ops;

use std::io::Read;
use anyhow::Result;
use clap::Parser;
use cli::{Cli, Command};

fn read_input(input: &Option<String>) -> Result<String> {
    match input {
        Some(s) => Ok(s.clone()),
        None => {
            let mut buf = String::new();
            std::io::stdin().read_to_string(&mut buf)?;
            Ok(buf.trim_end().to_string())
        }
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Command::Base64Encode { input } => {
            let data = read_input(input)?;
            println!("{}", ops::base64_op::encode(&data));
        }
        Command::Base64Decode { input } => {
            let data = read_input(input)?;
            println!("{}", ops::base64_op::decode(&data)?);
        }
        Command::HexEncode { input } => {
            let data = read_input(input)?;
            println!("{}", ops::hex_op::encode(&data));
        }
        Command::HexDecode { input } => {
            let data = read_input(input)?;
            println!("{}", ops::hex_op::decode(&data)?);
        }
        Command::UrlEncode { input } => {
            let data = read_input(input)?;
            println!("{}", ops::url_op::encode(&data));
        }
        Command::UrlDecode { input } => {
            let data = read_input(input)?;
            println!("{}", ops::url_op::decode(&data)?);
        }
        Command::Rot13 { input } => {
            let data = read_input(input)?;
            println!("{}", ops::rot13_op::apply(&data));
        }
        Command::Xor { key, input } => {
            let data = read_input(input)?;
            println!("{}", ops::xor_op::apply(&data, key)?);
        }
        Command::Md5 { input } => {
            let data = read_input(input)?;
            println!("{}", ops::hash_op::md5(&data));
        }
        Command::Sha1 { input } => {
            let data = read_input(input)?;
            println!("{}", ops::hash_op::sha1(&data));
        }
        Command::Sha256 { input } => {
            let data = read_input(input)?;
            println!("{}", ops::hash_op::sha256(&data));
        }
        Command::ExtractIps { input } => {
            let data = read_input(input)?;
            for ip in ops::extract_op::extract_ips(&data) {
                println!("{}", ip);
            }
        }
        Command::ExtractUrls { input } => {
            let data = read_input(input)?;
            for url in ops::extract_op::extract_urls(&data) {
                println!("{}", url);
            }
        }
        Command::ExtractEmails { input } => {
            let data = read_input(input)?;
            for email in ops::extract_op::extract_emails(&data) {
                println!("{}", email);
            }
        }
        Command::Stats { input } => {
            let data = read_input(input)?;
            let stats = ops::stats_op::compute(&data);
            println!("Character count:  {}", stats.char_count);
            println!("Word count:       {}", stats.word_count);
            println!("Line count:       {}", stats.line_count);
            println!("Byte count:       {}", stats.byte_count);
            println!("Letter count:     {}", stats.letter_count);
            println!("Digit count:      {}", stats.digit_count);
            println!("Whitespace count: {}", stats.whitespace_count);
            println!("Punctuation count: {}", stats.punctuation_count);
        }
        Command::Entropy { input } => {
            let data = read_input(input)?;
            println!("{}", ops::entropy_op::shannon(&data));
        }
    }

    Ok(())
}
