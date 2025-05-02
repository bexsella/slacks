/// This module contains the conversion code for dealing with clacks messages
/// converting latin numbers and letters into their clacks counterparts.

/// The semaphores for the clacks are represented by the nibbles in a byte.
/// the first 4 bits are the left shutters, and the last 4 the right.
pub type Semaphore = u8;

/// The conversions here are purely fictional, designed to be fairly
/// aesthetically pleasing when shown in sequence, save for the numbers
/// which simply count to 8, then add numbers on from 8 to add to 9 and
/// 0 (represented as 8 + 2).
pub fn from_char(c: char) -> Semaphore {
    match c {
        '0' => 0x82,
        '1' => 0x01,
        '2' => 0x02,
        '3' => 0x04,
        '4' => 0x08,
        '5' => 0x10,
        '6' => 0x20,
        '7' => 0x40,
        '8' => 0x80,
        '9' => 0x81,
        'A' | 'a' => 0x03,
        'B' | 'b' => 0x23,
        'C' | 'c' => 0x33,
        'D' | 'd' => 0x32,
        'E' | 'e' => 0x52,
        'F' | 'f' => 0x17,
        'G' | 'g' => 0x73,
        'H' | 'h' => 0x77,
        'I' | 'i' => 0xe1,
        'J' | 'j' => 0x78,
        'K' | 'k' => 0x96,
        'L' | 'l' => 0x8e,
        'M' | 'm' => 0x55,
        'N' | 'n' => 0xaa,
        'O' | 'o' => 0xcc,
        'P' | 'p' => 0x2e,
        'Q' | 'q' => 0xe2,
        'R' | 'r' => 0xd2,
        'S' | 's' => 0x5a,
        'T' | 't' => 0x1f,
        'U' | 'u' => 0x9f,
        'V' | 'v' => 0x57,
        'W' | 'w' => 0xdf,
        'X' | 'x' => 0x66,
        'Y' | 'y' => 0x2d,
        'Z' | 'z' => 0xa5,
        ' ' => 0x88,
        _ => 0xff
    }
}

pub fn to_char(s: Semaphore) -> char {
    match s {
        0x82 => '0',
        0x01 => '1',
        0x02 => '2',
        0x04 => '3',
        0x08 => '4',
        0x10 => '5',
        0x20 => '6',
        0x40 => '7',
        0x80 => '8',
        0x81 => '9',
        0x03 => 'A',
        0x23 => 'B',
        0x33 => 'C',
        0x32 => 'D',
        0x52 => 'E',
        0x17 => 'F',
        0x73 => 'G',
        0x77 => 'H',
        0xe1 => 'I',
        0x78 => 'J',
        0x96 => 'K',
        0x8e => 'L',
        0x55 => 'M',
        0xaa => 'N',
        0xcc => 'O',
        0x2e => 'P',
        0xe2 => 'Q',
        0xd2 => 'R',
        0x5a => 'S',
        0x1f => 'T',
        0x9f => 'U',
        0x57 => 'V',
        0xdf => 'W',
        0x66 => 'X',
        0x2d => 'Y',
        0xa5 => 'Z',
        0x88 => ' ',
        _ => '?'
    }
}

pub fn from_str(input: &str) -> Vec<Semaphore> {
    input.chars().map(|c| from_char(c)).collect()
}

pub fn to_str(input: Vec<Semaphore>) -> String {
    input.into_iter().map(|s| to_char(s)).collect()
}

pub fn pretty_print(input: Vec<Semaphore>) {
    for i in 0..4 {
        for s in input.iter() {
            let l = s >> i & 0x1;
            let r = s >> (i+4) & 0x1;
            print!("{}{} ", l, r); 
        }
        println!();
    }
}
