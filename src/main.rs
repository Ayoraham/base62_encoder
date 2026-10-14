use std::collections::HashMap;

use crate::DecodeError::{InvalidChar, Overflow};

enum DecodeError{
    InvalidChar(char, usize), 
    Overflow
}

fn encode(input: u128) -> String{
    let alphabet = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut conv_table = HashMap::new();

    for val in alphabet.char_indices(){
        conv_table.insert(val.0 as u128, val.1);
    }

    let mut input = input;
    let mut encoding = String::new();
    loop{
    let dividend  = input/62;
    let remainder = input %62;
    encoding.push(*conv_table.get(&remainder).unwrap());
    println!("{input}/62 = {dividend}r{remainder}");
    input /= 62;
    if dividend == 0{break}
}
    encoding.chars().rev().collect::<String>()
}

fn decode(input: &str) -> Result<u128, DecodeError>{
    let alphabet = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut conv_table = HashMap::new();

    for val in alphabet.char_indices(){
        conv_table.insert(val.1, val.0 as u128);
    };
    let mut decoded = 0_u128;
    for char in input.char_indices(){
        let lookup = *conv_table.get(&char.1).ok_or(DecodeError::InvalidChar(char.1, char.0))?;
        decoded = decoded.checked_mul(62).and_then(|n| n.checked_add(lookup)).ok_or(DecodeError::Overflow)?;}

    Ok(decoded)
}

fn main() {
    println!("{}", encode(u128::MAX));
    match decode("1geh263eq32"){
        Ok(num) => println!("{num}"),
        Err(InvalidChar(val, pos)) => println!("Invalid character {} at position {}", val, pos),
        Err(Overflow) => println!("Input too large")
    }

}
