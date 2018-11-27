use registers::flags_register::FlagsRegister;

#[derive(Default)]
pub struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: FlagsRegister,
    h: u8,
    l: u8,
}

impl Registers {
    pub fn get_bc(&self) -> u16 {
        (self.b as u16) << 8 | self.c as u16
    }

    pub fn set_bc(&mut self, value: u16) {
        self.b = ((value & 0xFF00) >> 8) as u8;
        self.c = (value & 0xFF) as u8;
    }
}

#[cfg(test)]
mod register_tests {
    use registers::flags_register::FlagsRegister;
    use registers::registers::Registers;

    #[test]
    fn getting_16_bit_from_b_and_c() {
        let regs = Registers {
            b: 0xF0,
            c: 0x0A,
            ..Default::default()
        };
        assert_eq!(regs.get_bc(), 0xF00A);
    }

    #[test]
    fn writing_16_bit_value_into_b_and_c() {
        let mut regs: Registers = Default::default();
        regs.set_bc(0xAABB);
        assert_eq!(regs.b, 0xAA);
        assert_eq!(regs.c, 0xBB);
    }

    #[test]
    fn using_flags_register() {
        let regs = Registers {
            f: FlagsRegister {
                zero: true,
                ..Default::default()
            },
            ..Default::default()
        };
        assert_eq!(u8::from(regs.f), 0b10000000);
    }
}
