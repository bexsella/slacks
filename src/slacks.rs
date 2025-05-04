/// This module contains the conversion code for dealing with clacks messages
/// converting latin numbers and letters into their clacks counterparts.

/// The semaphores for the clacks are represented by the nibbles in a byte.
/// the first 4 bits are the left shutters, and the last 4 the right.
pub enum Semaphore {
    AlphaNumeric(u8),
    ControlCode(u8)
}

impl Semaphore {
    /// The conversions here are purely fictional, designed to be fairly
    /// aesthetically pleasing when shown in sequence, save for the numbers
    /// which simply count to 8, then add numbers on from 8 to add to 9 and
    /// 0 (represented as 8 + 2).
    pub fn from_char(c: char) -> Self {
        match c {
            '0' => Semaphore::AlphaNumeric(0x82),
            '1' => Semaphore::AlphaNumeric(0x01),
            '2' => Semaphore::AlphaNumeric(0x02),
            '3' => Semaphore::AlphaNumeric(0x04),
            '4' => Semaphore::AlphaNumeric(0x08),
            '5' => Semaphore::AlphaNumeric(0x10),
            '6' => Semaphore::AlphaNumeric(0x20),
            '7' => Semaphore::AlphaNumeric(0x40),
            '8' => Semaphore::AlphaNumeric(0x80),
            '9' => Semaphore::AlphaNumeric(0x81),
            'A' | 'a' => Semaphore::AlphaNumeric(0x03),
            'B' | 'b' => Semaphore::AlphaNumeric(0x23),
            'C' | 'c' => Semaphore::AlphaNumeric(0x33),
            'D' | 'd' => Semaphore::AlphaNumeric(0x32),
            'E' | 'e' => Semaphore::AlphaNumeric(0x52),
            'F' | 'f' => Semaphore::AlphaNumeric(0x17),
            'G' | 'g' => Semaphore::AlphaNumeric(0x73),
            'H' | 'h' => Semaphore::AlphaNumeric(0x77),
            'I' | 'i' => Semaphore::AlphaNumeric(0xe1),
            'J' | 'j' => Semaphore::AlphaNumeric(0x78),
            'K' | 'k' => Semaphore::AlphaNumeric(0x96),
            'L' | 'l' => Semaphore::AlphaNumeric(0x8e),
            'M' | 'm' => Semaphore::AlphaNumeric(0x55),
            'N' | 'n' => Semaphore::AlphaNumeric(0xaa),
            'O' | 'o' => Semaphore::AlphaNumeric(0xcc),
            'P' | 'p' => Semaphore::AlphaNumeric(0x2e),
            'Q' | 'q' => Semaphore::AlphaNumeric(0xe2),
            'R' | 'r' => Semaphore::AlphaNumeric(0xd2),
            'S' | 's' => Semaphore::AlphaNumeric(0x5a),
            'T' | 't' => Semaphore::AlphaNumeric(0x1f),
            'U' | 'u' => Semaphore::AlphaNumeric(0x9f),
            'V' | 'v' => Semaphore::AlphaNumeric(0x57),
            'W' | 'w' => Semaphore::AlphaNumeric(0xdf),
            'X' | 'x' => Semaphore::AlphaNumeric(0x66),
            'Y' | 'y' => Semaphore::AlphaNumeric(0x2d),
            'Z' | 'z' => Semaphore::AlphaNumeric(0xa5),
            ' ' => Semaphore::AlphaNumeric(0x88),
            '[' => Semaphore::ControlCode(0x4e),
            ']' => Semaphore::ControlCode(0xe4),
            _ => Semaphore::ControlCode(0xff)
        }
    }

    pub fn to_char(self) -> char {
        match self {
            Semaphore::AlphaNumeric(0x82) => '0',
            Semaphore::AlphaNumeric(0x01) => '1',
            Semaphore::AlphaNumeric(0x02) => '2',
            Semaphore::AlphaNumeric(0x04) => '3',
            Semaphore::AlphaNumeric(0x08) => '4',
            Semaphore::AlphaNumeric(0x10) => '5',
            Semaphore::AlphaNumeric(0x20) => '6',
            Semaphore::AlphaNumeric(0x40) => '7',
            Semaphore::AlphaNumeric(0x80) => '8',
            Semaphore::AlphaNumeric(0x81) => '9',
            Semaphore::AlphaNumeric(0x03) => 'A',
            Semaphore::AlphaNumeric(0x23) => 'B',
            Semaphore::AlphaNumeric(0x33) => 'C',
            Semaphore::AlphaNumeric(0x32) => 'D',
            Semaphore::AlphaNumeric(0x52) => 'E',
            Semaphore::AlphaNumeric(0x17) => 'F',
            Semaphore::AlphaNumeric(0x73) => 'G',
            Semaphore::AlphaNumeric(0x77) => 'H',
            Semaphore::AlphaNumeric(0xe1) => 'I',
            Semaphore::AlphaNumeric(0x78) => 'J',
            Semaphore::AlphaNumeric(0x96) => 'K',
            Semaphore::AlphaNumeric(0x8e) => 'L',
            Semaphore::AlphaNumeric(0x55) => 'M',
            Semaphore::AlphaNumeric(0xaa) => 'N',
            Semaphore::AlphaNumeric(0xcc) => 'O',
            Semaphore::AlphaNumeric(0x2e) => 'P',
            Semaphore::AlphaNumeric(0xe2) => 'Q',
            Semaphore::AlphaNumeric(0xd2) => 'R',
            Semaphore::AlphaNumeric(0x5a) => 'S',
            Semaphore::AlphaNumeric(0x1f) => 'T',
            Semaphore::AlphaNumeric(0x9f) => 'U',
            Semaphore::AlphaNumeric(0x57) => 'V',
            Semaphore::AlphaNumeric(0xdf) => 'W',
            Semaphore::AlphaNumeric(0x66) => 'X',
            Semaphore::AlphaNumeric(0x2d) => 'Y',
            Semaphore::AlphaNumeric(0xa5) => 'Z',
            Semaphore::AlphaNumeric(0x88) => ' ',
            Semaphore::ControlCode(0x4e) => '[',
            Semaphore::ControlCode(0xe4) => ']',
            _ => '?'
        }
    }

    pub fn from_str(input: &str) -> Vec<Semaphore> {
        input.chars().map(|c| Semaphore::from_char(c)).collect()
    }

    pub fn to_str(input: Vec<Semaphore>) -> String {
        input.into_iter().map(|s| s.to_char()).collect()
    }

    pub fn pretty_print(input: Vec<Semaphore>) {
        for i in 0..4 {
            for s in input.iter() {
                match s {
                    Semaphore::AlphaNumeric(ch) => {
                        let l = ch >> i & 0x1u8;
                        let r = ch >> (i+4) & 0x1u8;

                        print!("{}{} ", l, r);
                    },

                    _ => {}
                }
            }
            println!();
        }
    }
}

pub struct ClacksMessage {
    msg: Vec<Semaphore>,
    addrs_from: u16,
    addrs_to: u16
}

impl ClacksMessage {
    fn new(msg: Vec<Semaphore>, addrs_from: u16, addrs_to: u16) -> Self {
        ClacksMessage {
            msg,
            addrs_from,
            addrs_to
        }
    }

    fn from_string(msg: String, addrs_from: u16, addrs_to: u16) -> Self {
        ClacksMessage {
            msg: Semaphore::from_str(msg.as_str()),
            addrs_from,
            addrs_to
        }
    }
}
