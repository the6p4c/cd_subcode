use crate::subcode::{SubcodeBit, SubcodeSymbol};

const CHANNEL_Q_LEN_SYMBOLS: usize = 96;
const DATA_Q_LEN_SYMBOLS: usize = 72;

const DATA_Q_FIRST: usize = 8;

fn bcd_digit_to_char(bcd_digit: u8) -> char {
    assert!(bcd_digit <= 9, "bcd digit out of range");

    (b'0' + bcd_digit) as char
}

fn two_digit_bcd_to_decimal(x: u8) -> u8 {
    let tens = (x >> 4) & 0xF;
    let ones = x & 0xF;

    assert!(tens <= 9, "bcd tens digit out of range");
    assert!(ones <= 9, "bcd ones digit out of range");

    tens * 10 + ones
}

fn six_bit_char_to_char(x: u8) -> char {
    match x {
        0..=9 => (b'0' + x) as char,
        17..=42 => (b'A' + (x - 17)) as char,
        _ => panic!("invalid six bit character {:#08b}", x),
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Address {
    Mode0,
    Mode1,
    Mode2,
    Mode3,
    // Mode 4 is listed in the IEC 60908 standard, though as a reference to IEC
    // 61104 - the CD Video standard. Pretty sure almost nobody uses CD Video
    // (now or ever) so it's not worth implementing.
    Unknown,
}

pub struct ChannelQ<'a> {
    symbols: &'a [SubcodeSymbol],
}

impl ChannelQ<'_> {
    pub fn new(symbols: &[SubcodeSymbol]) -> ChannelQ {
        assert_eq!(
            symbols.len(),
            CHANNEL_Q_LEN_SYMBOLS,
            "q channel data must be exactly {} symbols long",
            CHANNEL_Q_LEN_SYMBOLS
        );

        ChannelQ { symbols }
    }

    fn value(&self, first: usize, len: usize) -> u32 {
        assert!(len <= 32, "value too wide to represent");
        assert!(
            first < CHANNEL_Q_LEN_SYMBOLS,
            "value first index out of range"
        );
        let last = first + len - 1;
        assert!(
            last < CHANNEL_Q_LEN_SYMBOLS,
            "value too wide for first index"
        );

        // Powers of two required for conversion, moving from MSB first to LSB
        // last
        // (len - 1 - p) changes range from 0..len to (len-1)..0
        let powers = (0..len).map(|p| 1u32 << (len - 1 - p));

        self.symbols[first..(first + len)]
            .iter()
            .map(|s| s.get_bit_value(SubcodeBit::Q) as u32)
            .zip(powers)
            .map(|(value, power)| value * power)
            .sum()
    }

    pub(super) fn value_data_q(&self, first: usize, len: usize) -> u32 {
        assert!(len <= 32, "value too wide to represent");
        assert!(first < DATA_Q_LEN_SYMBOLS, "value first index out of range");
        let last = first + len - 1;
        assert!(last < DATA_Q_LEN_SYMBOLS, "value too wide for first index");

        self.value(DATA_Q_FIRST + first, len)
    }

    pub fn control(&self) -> u8 {
        self.value(0, 4) as u8
    }

    pub fn adr(&self) -> Address {
        let adr = self.value(4, 4) as u8;

        match adr {
            0b0000 => Address::Mode0,
            0b0001 => Address::Mode1,
            0b0010 => Address::Mode2,
            0b0011 => Address::Mode3,
            _ => Address::Unknown,
        }
    }

    pub fn crc(&self) -> u16 {
        self.value(80, 16) as u16
    }

    pub fn as_mode1(&self) -> Option<ChannelQMode1<'_>> {
        if self.adr() == Address::Mode1 {
            Some(ChannelQMode1(self))
        } else {
            None
        }
    }

    pub fn as_mode2(&self) -> Option<ChannelQMode2<'_>> {
        if self.adr() == Address::Mode2 {
            Some(ChannelQMode2(self))
        } else {
            None
        }
    }

    pub fn as_mode3(&self) -> Option<ChannelQMode3<'_>> {
        if self.adr() == Address::Mode3 {
            Some(ChannelQMode3(self))
        } else {
            None
        }
    }
}

pub struct ChannelQMode1<'a>(&'a ChannelQ<'a>);

impl ChannelQMode1<'_> {
    pub fn tno(&self) -> u8 {
        two_digit_bcd_to_decimal(self.0.value_data_q(0, 8) as u8)
    }

    pub fn point(&self) -> u8 {
        two_digit_bcd_to_decimal(self.0.value_data_q(8, 8) as u8)
    }

    pub fn min(&self) -> u8 {
        two_digit_bcd_to_decimal(self.0.value_data_q(16, 8) as u8)
    }

    pub fn sec(&self) -> u8 {
        two_digit_bcd_to_decimal(self.0.value_data_q(24, 8) as u8)
    }

    pub fn frame(&self) -> u8 {
        two_digit_bcd_to_decimal(self.0.value_data_q(32, 8) as u8)
    }

    pub fn a_p_min(&self) -> u8 {
        two_digit_bcd_to_decimal(self.0.value_data_q(48, 8) as u8)
    }

    pub fn a_p_sec(&self) -> u8 {
        two_digit_bcd_to_decimal(self.0.value_data_q(56, 8) as u8)
    }

    pub fn a_p_frame(&self) -> u8 {
        two_digit_bcd_to_decimal(self.0.value_data_q(64, 8) as u8)
    }
}

pub struct ChannelQMode2<'a>(&'a ChannelQ<'a>);

impl ChannelQMode2<'_> {
    fn field_n(&self, n: usize) -> u8 {
        assert!(n >= 1 && n <= 13, "invalid N field");

        self.0.value_data_q((n - 1) * 4, 4) as u8
    }

    pub fn catalogue_number(&self) -> String {
        (1..=13)
            .map(|n| self.field_n(n))
            .map(bcd_digit_to_char)
            .collect()
    }
}

pub struct ChannelQMode3<'a>(&'a ChannelQ<'a>);

impl ChannelQMode3<'_> {
    fn field_i(&self, i: usize) -> u8 {
        match i {
            1..=5 => self.0.value_data_q((i - 1) * 6, 6) as u8,
            6..=12 => self.0.value_data_q(5 * 6 + 2 + (i - 6) * 4, 4) as u8,
            _ => panic!("invalid I field"),
        }
    }

    pub fn country_code(&self) -> String {
        (1..=2)
            .map(|i| self.field_i(i))
            .map(six_bit_char_to_char)
            .collect()
    }

    pub fn owner_code(&self) -> String {
        (3..=5)
            .map(|i| self.field_i(i))
            .map(six_bit_char_to_char)
            .collect()
    }

    pub fn year(&self) -> String {
        (6..=7)
            .map(|i| self.field_i(i))
            .map(bcd_digit_to_char)
            .collect()
    }

    pub fn serial_number(&self) -> String {
        (8..=12)
            .map(|i| self.field_i(i))
            .map(bcd_digit_to_char)
            .collect()
    }

    pub fn isrc(&self) -> String {
        format!(
            "{}{}{}{}",
            self.country_code(),
            self.owner_code(),
            self.year(),
            self.serial_number(),
        )
    }
}
