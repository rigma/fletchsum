// This file is part of fletchsum.
//
//  fletchsum is free software: you can redistribute it and/or modify
//  it under the terms of the GNU General Public License as published by
//  the Free Software Foundation, either version 3 of the License, or
//  (at your option) any later version.
//
//  fletchsum is distributed in the hope that it will be useful,
//  but WITHOUT ANY WARRANTY; without even the implied warranty of
//  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//  GNU General Public License for more details.
//
//  You should have received a copy of the GNU General Public License
//  along with Foobar.  If not, see <https://www.gnu.org/licenses/>

#[macro_use]
extern crate clap;
extern crate fletchsum;

use std::fs::File;
use std::io::{self, Read, Result};
use std::process;

use clap::App;
use fletchsum::*;

fn main() {
    // Loading the command line arguments schema of the app
    let args = load_yaml!("cli.yml");
    let args = App::from_yaml(args).get_matches();

    // Guessing if the app must be running into the binary mode or not
    let binary_mode = if let Some(_) = args.value_of("binary") {
        true
    } else {
        false
    };

    // Checking if the FILE argument is given
    match args.value_of("FILE") {
        Some(path) => {
            // If we got a file argument, if it's "-" we'll read the standard input
            // or we'll read the file path which is given by the user
            if path == "-" {
                standard_input_checksum(binary_mode);
            } else {
                file_checksum(path, binary_mode);
            }
        },
        None => standard_input_checksum(binary_mode),
    };
}

fn standard_input_checksum(binary_mode: bool) {
    let checksum: u64;

    if binary_mode {
        // If we are in binary mode, we'll compute the checksum from a u8 vector
        checksum = match read_bytes_from_standard_input() {
            Ok(data) => fletcher_checksum_bytes(&data),
            Err(err) => {
                eprintln!("Error: {}", err);
                process::exit(1);
            },
        };
    } else {
        // Otherwise, we'll compute the checksum from a String struct
        checksum = match read_standard_input() {
            Ok(data) => fletcher_checksum_str(&data),
            Err(err) => {
                eprintln!("Error: {}", err);
                process::exit(1);
            },
        };
    }

    // At the end we display the checksum in hexadecimal
    println!("{:x}", checksum);
}

// TODO: find a way to have only one function to compute the checksum
fn file_checksum(path: &str, binary_mode: bool) {
    let checksum: u64;

    if binary_mode {
        // If we are in binary mode, we'll compute the checksum from a u8 vector
        checksum = match read_bytes_from_file(path) {
            Ok(data) => fletcher_checksum_bytes(&data),
            Err(err) => {
                eprintln!("Error: {}", err);
                process::exit(1);
            }
        };
    } else {
        // Otherwise, we'll compute the checksum from a String struct
        checksum = match read_file(path) {
            Ok(data) => fletcher_checksum_str(&data),
            Err(err) => {
                eprintln!("Error: {}", err);
                process::exit(1);
            }
        };
    }

    println!("{:x}", checksum);
}

fn read_bytes_from_standard_input() -> Result<Vec<u8>> {
    let mut buffer = Vec::new();
    
    io::stdin().read_to_end(&mut buffer)?;
    Ok(buffer)
}

fn read_standard_input() -> Result<String> {
    let mut buffer = String::new();

    io::stdin().read_to_string(&mut buffer)?;
    Ok(buffer)
}

fn read_bytes_from_file(path: &str) -> Result<Vec<u8>> {
    let mut f = File::open(path)?;
    let mut buffer = Vec::new();

    f.read_to_end(&mut buffer)?;
    Ok(buffer)
}

fn read_file(path: &str) -> Result<String> {
    let mut f = File::open(path)?;
    let mut buffer = String::new();

    f.read_to_string(&mut buffer)?;
    Ok(buffer)
}
