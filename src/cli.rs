use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "rustchef")]
#[command(about = "A CyberChef-inspired CLI tool for data transformations", long_about = None)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Encode data to Base64
    Base64Encode {
        /// Input string (reads from stdin if not provided)
        input: Option<String>,
    },
    /// Decode Base64 data
    Base64Decode {
        /// Input string (reads from stdin if not provided)
        input: Option<String>,
    },
    /// Encode data to hexadecimal
    HexEncode {
        /// Input string (reads from stdin if not provided)
        input: Option<String>,
    },
    /// Decode hexadecimal data
    HexDecode {
        /// Input string (reads from stdin if not provided)
        input: Option<String>,
    },
    /// URL-encode a string
    UrlEncode {
        /// Input string (reads from stdin if not provided)
        input: Option<String>,
    },
    /// URL-decode a string
    UrlDecode {
        /// Input string (reads from stdin if not provided)
        input: Option<String>,
    },
    /// Apply ROT13 transformation
    Rot13 {
        /// Input string (reads from stdin if not provided)
        input: Option<String>,
    },
    /// XOR data with a key
    Xor {
        /// XOR key (required)
        #[arg(long, short)]
        key: String,
        /// Input string (reads from stdin if not provided)
        input: Option<String>,
    },
    /// Compute MD5 hash
    Md5 {
        /// Input string (reads from stdin if not provided)
        input: Option<String>,
    },
    /// Compute SHA1 hash
    Sha1 {
        /// Input string (reads from stdin if not provided)
        input: Option<String>,
    },
    /// Compute SHA256 hash
    Sha256 {
        /// Input string (reads from stdin if not provided)
        input: Option<String>,
    },
    /// Extract IPv4 and IPv6 addresses
    ExtractIps {
        /// Input string (reads from stdin if not provided)
        input: Option<String>,
    },
    /// Extract URLs from text
    ExtractUrls {
        /// Input string (reads from stdin if not provided)
        input: Option<String>,
    },
    /// Extract email addresses
    ExtractEmails {
        /// Input string (reads from stdin if not provided)
        input: Option<String>,
    },
    /// Compute text statistics
    Stats {
        /// Input string (reads from stdin if not provided)
        input: Option<String>,
    },
    /// Compute Shannon entropy
    Entropy {
        /// Input string (reads from stdin if not provided)
        input: Option<String>,
    },
}
