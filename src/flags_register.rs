extern crate std;

#[derive(Default)]
pub struct FlagsRegister {
    pub zero: bool,
    pub substract: bool,
    pub half_carry: bool,
    pub carry: bool,
}

const ZERO_POSITION_IN_BYTE: u8 = 7;
const SUBSTRACT_POSITION_IN_BYTE: u8 = 6;
const HALF_CARRY_POSITION_IN_BYTE: u8 = 5;
const CARRY_POSITION_IN_BYTE: u8 = 4;

impl std::convert::From<FlagsRegister> for u8 {
    fn from(flag: FlagsRegister) -> u8 {
        (if flag.zero { 1 } else { 0 }) << ZERO_POSITION_IN_BYTE
            | (if flag.substract { 1 } else { 0 }) << SUBSTRACT_POSITION_IN_BYTE
            | (if flag.half_carry { 1 } else { 0 }) << HALF_CARRY_POSITION_IN_BYTE
            | (if flag.carry { 1 } else { 0 }) << CARRY_POSITION_IN_BYTE
    }
}

#[cfg(test)]
mod flags_register_tests {
    use FlagsRegister;

    #[test]
    fn convert_zero_flag_into_u8() {
        let flag_reg = FlagsRegister {
            zero: true,
            substract: false,
            half_carry: false,
            carry: false,
        };
        assert_eq!(0b10000000, u8::from(flag_reg));
    }

    #[test]
    fn convert_substract_flag_into_u8() {
        let flag_reg = FlagsRegister {
            zero: false,
            substract: true,
            half_carry: false,
            carry: false,
        };
        assert_eq!(0b01000000, u8::from(flag_reg));
    }

    #[test]
    fn convert_half_carry_into_u8() {
        let flag_reg = FlagsRegister {
            zero: false,
            substract: false,
            half_carry: true,
            carry: false,
        };
        assert_eq!(0b00100000, u8::from(flag_reg));
    }

    #[test]
    fn convert_carry_flag_into_u8() {
        let flag_reg = FlagsRegister {
            zero: false,
            substract: false,
            half_carry: false,
            carry: true,
        };
        assert_eq!(0b00010000, u8::from(flag_reg));
    }

    #[test]
    fn convert_some_into_u8() {
        let flag_reg = FlagsRegister {
            zero: true,
            substract: false,
            half_carry: false,
            carry: true,
        };
        assert_eq!(0b10010000, u8::from(flag_reg));
    }
}
