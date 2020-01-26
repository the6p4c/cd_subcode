use std::fmt;

#[derive(Debug)]
pub enum SubcodeBit {
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
}

pub struct SubcodeSymbol(pub u8);

impl SubcodeSymbol {
    /// Retrieves the value of a bit within the subcode symbol.
    pub fn get_bit_value(&self, subcode_bit: SubcodeBit) -> u8 {
        let bit = match subcode_bit {
            SubcodeBit::P => 7,
            SubcodeBit::Q => 6,
            SubcodeBit::R => 5,
            SubcodeBit::S => 4,
            SubcodeBit::T => 3,
            SubcodeBit::U => 2,
            SubcodeBit::V => 1,
            SubcodeBit::W => 0,
        };

        (self.0 >> bit) & 1
    }

    /// Retrieves the value of a bit within the subcode symbol, as a bool.
    ///
    /// Returns true for a 1 bit, false for a 0 bit.
    pub fn get_bit(&self, subcode_bit: SubcodeBit) -> bool {
        match self.get_bit_value(subcode_bit) {
            0 => false,
            1 => true,
            _ => panic!("bit value was not 0 or 1 (this shouldn't happen)"),
        }
    }
}

impl fmt::Debug for SubcodeSymbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Only show the bit symbol letters if at least one bit is set, to avoid
        // showing an extra space when it's unneeded
        let bit_string = if self.0 != 0 {
            format!(
                " {}{}{}{}{}{}{}{}",
                if self.get_bit(SubcodeBit::P) { "P" } else { "" },
                if self.get_bit(SubcodeBit::Q) { "Q" } else { "" },
                if self.get_bit(SubcodeBit::R) { "R" } else { "" },
                if self.get_bit(SubcodeBit::S) { "S" } else { "" },
                if self.get_bit(SubcodeBit::T) { "T" } else { "" },
                if self.get_bit(SubcodeBit::U) { "U" } else { "" },
                if self.get_bit(SubcodeBit::V) { "V" } else { "" },
                if self.get_bit(SubcodeBit::W) { "W" } else { "" },
            )
        } else {
            "".into()
        };

        write!(f, "SubcodeSymbol({:#010b}{})", self.0, bit_string)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn subcode_symbol_get_bit_value_returns_correct_values() {
        let subcode_symbol = SubcodeSymbol(0b0000_0000);
        assert_eq!(subcode_symbol.get_bit_value(SubcodeBit::P), 0);
        assert_eq!(subcode_symbol.get_bit_value(SubcodeBit::Q), 0);
        assert_eq!(subcode_symbol.get_bit_value(SubcodeBit::R), 0);
        assert_eq!(subcode_symbol.get_bit_value(SubcodeBit::S), 0);
        assert_eq!(subcode_symbol.get_bit_value(SubcodeBit::T), 0);
        assert_eq!(subcode_symbol.get_bit_value(SubcodeBit::U), 0);
        assert_eq!(subcode_symbol.get_bit_value(SubcodeBit::V), 0);
        assert_eq!(subcode_symbol.get_bit_value(SubcodeBit::W), 0);

        let subcode_symbol = SubcodeSymbol(0b1111_1111);
        assert_eq!(subcode_symbol.get_bit_value(SubcodeBit::P), 1);
        assert_eq!(subcode_symbol.get_bit_value(SubcodeBit::Q), 1);
        assert_eq!(subcode_symbol.get_bit_value(SubcodeBit::R), 1);
        assert_eq!(subcode_symbol.get_bit_value(SubcodeBit::S), 1);
        assert_eq!(subcode_symbol.get_bit_value(SubcodeBit::T), 1);
        assert_eq!(subcode_symbol.get_bit_value(SubcodeBit::U), 1);
        assert_eq!(subcode_symbol.get_bit_value(SubcodeBit::V), 1);
        assert_eq!(subcode_symbol.get_bit_value(SubcodeBit::W), 1);

        let subcode_symbol = SubcodeSymbol(0b1010_1010);
        assert_eq!(subcode_symbol.get_bit_value(SubcodeBit::P), 1);
        assert_eq!(subcode_symbol.get_bit_value(SubcodeBit::Q), 0);
        assert_eq!(subcode_symbol.get_bit_value(SubcodeBit::R), 1);
        assert_eq!(subcode_symbol.get_bit_value(SubcodeBit::S), 0);
        assert_eq!(subcode_symbol.get_bit_value(SubcodeBit::T), 1);
        assert_eq!(subcode_symbol.get_bit_value(SubcodeBit::U), 0);
        assert_eq!(subcode_symbol.get_bit_value(SubcodeBit::V), 1);
        assert_eq!(subcode_symbol.get_bit_value(SubcodeBit::W), 0);

        let subcode_symbol = SubcodeSymbol(0b0101_0101);
        assert_eq!(subcode_symbol.get_bit_value(SubcodeBit::P), 0);
        assert_eq!(subcode_symbol.get_bit_value(SubcodeBit::Q), 1);
        assert_eq!(subcode_symbol.get_bit_value(SubcodeBit::R), 0);
        assert_eq!(subcode_symbol.get_bit_value(SubcodeBit::S), 1);
        assert_eq!(subcode_symbol.get_bit_value(SubcodeBit::T), 0);
        assert_eq!(subcode_symbol.get_bit_value(SubcodeBit::U), 1);
        assert_eq!(subcode_symbol.get_bit_value(SubcodeBit::V), 0);
        assert_eq!(subcode_symbol.get_bit_value(SubcodeBit::W), 1);
    }

    #[test]
    fn subcode_symbol_get_bit_returns_correct_values() {
        let subcode_symbol = SubcodeSymbol(0b0000_0000);
        assert_eq!(subcode_symbol.get_bit(SubcodeBit::P), false);
        assert_eq!(subcode_symbol.get_bit(SubcodeBit::Q), false);
        assert_eq!(subcode_symbol.get_bit(SubcodeBit::R), false);
        assert_eq!(subcode_symbol.get_bit(SubcodeBit::S), false);
        assert_eq!(subcode_symbol.get_bit(SubcodeBit::T), false);
        assert_eq!(subcode_symbol.get_bit(SubcodeBit::U), false);
        assert_eq!(subcode_symbol.get_bit(SubcodeBit::V), false);
        assert_eq!(subcode_symbol.get_bit(SubcodeBit::W), false);

        let subcode_symbol = SubcodeSymbol(0b1111_1111);
        assert_eq!(subcode_symbol.get_bit(SubcodeBit::P), true);
        assert_eq!(subcode_symbol.get_bit(SubcodeBit::Q), true);
        assert_eq!(subcode_symbol.get_bit(SubcodeBit::R), true);
        assert_eq!(subcode_symbol.get_bit(SubcodeBit::S), true);
        assert_eq!(subcode_symbol.get_bit(SubcodeBit::T), true);
        assert_eq!(subcode_symbol.get_bit(SubcodeBit::U), true);
        assert_eq!(subcode_symbol.get_bit(SubcodeBit::V), true);
        assert_eq!(subcode_symbol.get_bit(SubcodeBit::W), true);

        let subcode_symbol = SubcodeSymbol(0b1010_1010);
        assert_eq!(subcode_symbol.get_bit(SubcodeBit::P), true);
        assert_eq!(subcode_symbol.get_bit(SubcodeBit::Q), false);
        assert_eq!(subcode_symbol.get_bit(SubcodeBit::R), true);
        assert_eq!(subcode_symbol.get_bit(SubcodeBit::S), false);
        assert_eq!(subcode_symbol.get_bit(SubcodeBit::T), true);
        assert_eq!(subcode_symbol.get_bit(SubcodeBit::U), false);
        assert_eq!(subcode_symbol.get_bit(SubcodeBit::V), true);
        assert_eq!(subcode_symbol.get_bit(SubcodeBit::W), false);

        let subcode_symbol = SubcodeSymbol(0b0101_0101);
        assert_eq!(subcode_symbol.get_bit(SubcodeBit::P), false);
        assert_eq!(subcode_symbol.get_bit(SubcodeBit::Q), true);
        assert_eq!(subcode_symbol.get_bit(SubcodeBit::R), false);
        assert_eq!(subcode_symbol.get_bit(SubcodeBit::S), true);
        assert_eq!(subcode_symbol.get_bit(SubcodeBit::T), false);
        assert_eq!(subcode_symbol.get_bit(SubcodeBit::U), true);
        assert_eq!(subcode_symbol.get_bit(SubcodeBit::V), false);
        assert_eq!(subcode_symbol.get_bit(SubcodeBit::W), true);
    }

    #[test]
    fn subcode_symbol_zero_debug_writes_correct_value() {
        let s = format!("{:?}", SubcodeSymbol(0b0000_0000));

        assert_eq!(s, "SubcodeSymbol(0b00000000)");
    }

    #[test]
    fn subcode_symbol_all_debug_writes_correct_value() {
        let s = format!("{:?}", SubcodeSymbol(0b1111_1111));

        assert_eq!(s, "SubcodeSymbol(0b11111111 PQRSTUVW)");
    }

    #[test]
    fn subcode_symbol_some_debug_writes_correct_value() {
        let s = format!("{:?}", SubcodeSymbol(0b1001_0110));

        assert_eq!(s, "SubcodeSymbol(0b10010110 PSUV)");
    }
}
