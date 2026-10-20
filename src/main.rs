use crate::DecodeError::{InvalidChar, Overflow};
use clap::{Parser};
use std::num::ParseIntError;
use std::{collections::HashMap, process};

const ALPHABET: &str = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

enum DecodeError {
    InvalidChar(char, usize),
    Overflow,
}

enum ParseError {
    InvalidOperation(String),
    InvalidNumber(ParseIntError),
}

enum Operation {
    Encode(u128),
    Decode(String),
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Operation you want to perform (Either Encode or Decode)
    #[arg(short, long)]
    operation: String,

    /// Value you want to convert
    #[arg(short, long)]
    value: String,
}

impl Args {
    fn init(&self) -> Result<Operation, ParseError> {
        let op_low = self.operation.to_lowercase();
        match op_low.as_str() {
            "encode" => {
                let val = self
                    .value
                    .parse::<u128>()
                    .map_err(ParseError::InvalidNumber)?;
                Ok(Operation::Encode(val))
            }
            "decode" => Ok(Operation::Decode(self.value.clone())),
            _ => Err(ParseError::InvalidOperation(op_low)),
        }
    }
}

fn encode(input: u128) -> String {
    let mut input = input;
    let mut encoding = String::new();
    loop {
        let dividend = input / 62;
        let remainder = input % 62;
        encoding.push(ALPHABET.as_bytes()[remainder as usize] as char);
        input /= 62;
        if dividend == 0 {
            break;
        }
    }
    encoding.chars().rev().collect::<String>()
}

fn decode(input: String) -> Result<u128, DecodeError> {
    let mut conv_table = HashMap::new();

    for val in ALPHABET.char_indices() {
        conv_table.insert(val.1, val.0 as u128);
    }
    let mut decoded = 0_u128;
    for char in input.char_indices() {
        let lookup = *conv_table
            .get(&char.1)
            .ok_or(DecodeError::InvalidChar(char.1, char.0))?;
        decoded = decoded
            .checked_mul(62)
            .and_then(|n| n.checked_add(lookup))
            .ok_or(DecodeError::Overflow)?;
    }

    Ok(decoded)
}

fn main() {
    let args = Args::parse();
    match args.init() {
        Ok(val) => {
            match val {
                Operation::Encode(no) => println!("{}", encode(no)),
                Operation::Decode(value) => match decode(value){
                    Ok(no) => println!("{no}"),
                    Err(err) => match err {
                        InvalidChar(val, pos) => {eprintln!("Invalid character {} at position {}", val, pos);process::exit(1)},
                        Overflow => {eprintln!("Input too large");
                                        process::exit(1)}
                    }
                }
            }
        }
        Err(error_type) => match error_type {
            ParseError::InvalidOperation(operation) => {
                eprintln!("Operation '{operation}' not recognized\nEnter either Encode or Decode!");
                process::exit(1)
            }
            ParseError::InvalidNumber(kind) => {
                eprintln!("There was an error: {}", kind);
                process::exit(1)
            }
        },
    };
    
}
