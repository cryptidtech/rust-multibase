use std::fmt;
use std::io::{self, Read, Write};
use std::str::FromStr;

use anyhow::{Context, Error, Result, anyhow};
use clap::Parser;
use multi_base::Base;

#[derive(Parser, Debug)]
struct Opts {
    /// The mode
    #[command(subcommand)]
    mode: Mode,
}

#[derive(Parser, Debug)]
enum Mode {
    /// Encode data to multibase format
    Encode {
        /// The base to use for encoding.
        #[arg(short = 'b', long = "base", default_value = "base58btc")]
        base: StrBase,
        /// The data to encode. Reads from stdin if not provided.
        #[arg(short = 'i', long = "input")]
        input: Option<String>,
    },
    /// Decode multibase-encoded data
    Decode {
        /// The data to decode. Reads from stdin if not provided.
        #[arg(short = 'i', long = "input")]
        input: Option<String>,
    },
}

fn main() -> Result<()> {
    env_logger::init();
    let opts = Opts::parse();
    match opts.mode {
        Mode::Encode { base, input } => {
            let input_bytes = if let Some(s) = input {
                s.into_bytes()
            } else {
                let mut buf = Vec::new();
                io::stdin()
                    .read_to_end(&mut buf)
                    .context("Failed to read input from stdin")?;
                buf
            };
            encode(base, &input_bytes)
        }
        Mode::Decode { input } => {
            let input_str = if let Some(s) = input {
                s
            } else {
                let mut buf = String::new();
                io::stdin()
                    .read_to_string(&mut buf)
                    .context("Failed to read input from stdin")?;
                buf
            };
            decode(&input_str)
        }
    }
}

/// A wrapper around Base that provides string conversion.
///
/// This type enables parsing base names from command-line arguments
/// and displaying them in help text.
#[derive(Debug, Clone)]
struct StrBase(Base);

/// Generates Display and `FromStr` implementations for `StrBase` using a single source of truth.
///
/// This macro eliminates code duplication by defining the Base ↔ string mappings once
/// and generating both trait implementations from that definition.
macro_rules! impl_base_string_conversion {
    ( $($variant:ident => $string:literal),* $(,)? ) => {
        impl fmt::Display for StrBase {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let base_str = match self.0 {
                    $( Base::$variant => $string, )*
                };
                write!(f, "{}", base_str)
            }
        }

        impl FromStr for StrBase {
            type Err = Error;

            fn from_str(base_str: &str) -> Result<Self, Self::Err> {
                let base = match base_str {
                    $( $string => Ok(Base::$variant), )*
                    _ => {
                        let available = [ $($string),* ].join(", ");
                        return Err(anyhow!(
                            "Unknown base: {:?}\n\nAvailable bases:\n  {}",
                            base_str,
                            available
                        ));
                    }
                };
                base.map(Self)
            }
        }
    };
}

impl_base_string_conversion! {
    Identity => "identity",
    Base2 => "base2",
    Base8 => "base8",
    Base10 => "base10",
    Base16Lower => "base16",
    Base16Upper => "base16upper",
    Base32HexLower => "base32hex",
    Base32HexUpper => "base32hexupper",
    Base32HexPadLower => "base32hexpad",
    Base32HexPadUpper => "base32hexpadupper",
    Base32Lower => "base32",
    Base32Upper => "base32upper",
    Base32PadLower => "base32pad",
    Base32PadUpper => "base32padupper",
    Base32Z => "base32z",
    Base36Lower => "base36lower",
    Base36Upper => "base36upper",
    Base58Flickr => "base58flickr",
    Base58Btc => "base58btc",
    Base64 => "base64",
    Base64Pad => "base64pad",
    Base64Url => "base64url",
    Base64UrlPad => "base64urlpad",
    Base256Emoji => "base256emoji",
}

impl From<StrBase> for Base {
    fn from(base: StrBase) -> Self {
        base.0
    }
}

fn encode(base: StrBase, input: &[u8]) -> Result<()> {
    log::debug!("Encode {input:?} with {base}");
    let result = multi_base::encode(base.into(), input);
    print!("{result}");
    io::stdout()
        .flush()
        .context("Failed to write encoded output to stdout")?;
    Ok(())
}

fn decode(input: &str) -> Result<()> {
    log::debug!("Decode {input:?}");
    let (detected_base, result) = multi_base::decode(input, true)
        .context("Failed to decode input. Make sure it starts with a valid multibase prefix")?;

    log::debug!("Detected base: {detected_base:?}");
    io::stdout()
        .write_all(&result)
        .context("Failed to write decoded output to stdout")?;
    io::stdout().flush().context("Failed to flush stdout")?;
    Ok(())
}
