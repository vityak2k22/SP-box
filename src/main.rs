#![allow(non_snake_case)]

use std::io;
mod s_box_rndl;

fn main () {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    if input.len() > 8 {
        input = input[0..8].to_string();
    }
    println!("S-Box:\n\tbin: {:0b}\n\thex: {:0x}", s_box(&input[..], true), s_box(&input[..], true));
    println!("InvS-box:\n\tbin: {:0b}\n\thex: {:0x}", s_box(&input[..], false), s_box(&input[..], false));

    assert_eq!(0b00111100, p_box("10001101", true));
}

fn s_box (bin_char: &str, is_direct: bool) -> u8 {
    let character = u8::from_str_radix(bin_char, 2).unwrap();
    if is_direct == true {
        s_box_rndl::SBOX_RNDL[character as usize]
    }
    else {
        s_box_rndl::SBOX_INV_RNDL[character as usize]
    }
}

fn p_box (bin_char: &str, is_direct: bool) -> u8 {
    let character = u8::from_str_radix(bin_char, 2).unwrap();
    let mut result_character = [0u8; 8];
    if is_direct == true {
        result_character[2] = character >> 7 << 7;
        result_character[1] = character >> 6 << 7 >> 1;
        result_character[4] = character >> 5 << 7 >> 2;
        result_character[7] = character >> 4 << 7 >> 3;
        result_character[5] = character >> 3 << 7 >> 4;
        result_character[0] = character >> 2 << 7 >> 5;
        result_character[6] = character >> 1 << 7 >> 6;
        result_character[3] = character >> 0 << 7 >> 7;
    }
    else {
        
    }
    for i in 0..8 {
        println!("{:b}", result_character[i]);
    }
    result_character[0] | result_character[1] | result_character[2] | result_character[3] |
    result_character[4] | result_character[5] | result_character[6] | result_character[7]
}

#[test]
fn test_s_box () {
    assert_eq!(s_box("10001101", true), 0b0101_1101);
    assert_eq!(s_box("01011101", false), 0b1000_1101);
    assert_eq!(s_box("00000000", true), 0x63);
    assert_eq!(s_box("00000000", false), 0x52);
    assert_eq!(s_box("11111111", true), 0x16);
    assert_eq!(s_box("11111111", false), 0x7d);
}

#[test]
fn test_p_box () {
    //assert_eq!(0b00111100, p_box("10001101", true));
}