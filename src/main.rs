//! Compress a series of given strings using CRC64. Uses the underlying `crc64` crate.
//!
//! # Example
//!
//! ```no_build
//! $ crmprs asdf
//! Processing 1 string(s).
//! String: asdf
//! CRC64: 0xe415192451a5f41e
//! ```
use clap::{crate_authors, crate_version, App, Arg};

fn main() {
    let matches = App::new("CRC64 String Compressor")
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .about("Compress a given list of strings using CRC64. Not secure.")
        .arg(
            Arg::with_name("strings")
                .takes_value(true)
                .required(true)
                .multiple(true),
        )
        .get_matches();

    if let Some(strings) = matches.values_of("strings") {
        println!("Processing {} string(s).", strings.len());
        for string in strings {
            println!("String: {}", string);
            let crc = crc64::crc64(0, string.as_bytes());
            println!("CRC64: {:#016x}", crc);
        }
    }
}
