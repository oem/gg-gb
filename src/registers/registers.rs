use registers::flags_register::FlagsRegister;

#[derive(Default)]
pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: FlagsRegister,
    pub h: u8,
    pub l: u8,
}

impl Registers {
    pub fn get_bc(&self) -> u16 {
        (self.b as u16) << 8 | self.c as u16
    }

    pub fn set_bc(&mut self, value: u16) {
        self.b = ((value & 0xFF00) >> 8) as u8;
        self.c = (value & 0xFF) as u8;
    }

    pub fn get_de(&self) -> u16 {
        (self.d as u16) << 8 | self.e as u16
    }

    pub fn set_de(&mut self, value: u16) {
        self.d = ((value & 0xFF00) >> 8) as u8;
        self.e = (value & 0xFF) as u8;
    }
}

#[cfg(test)]
mod tests {
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
    fn getting_16_bit_from_d_and_e() {
        let regs = Registers {
            d: 0xF0,
            e: 0x0A,
            ..Default::default()
        };
        assert_eq!(regs.get_de(), 0xF00A);
    }

    #[test]
    fn writing_16_bit_value_into_d_and_e() {
        let mut regs: Registers = Default::default();
        regs.set_de(0xAABB);
        assert_eq!(regs.d, 0xAA);
        assert_eq!(regs.e, 0xBB);
    }
}
