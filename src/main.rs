#[derive(Default)]
struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
    h: u8,
    l: u8,
}

impl Registers {
    fn get_bc(&self) -> u16 {
        (self.b as u16) << 8 | self.c as u16
    }

    fn set_bc(&mut self, value: u16) {
        self.b = (value & 0xFF00 >> 8) as u8;
        self.c = (value & 0xFF) as u8;
    }
}

#[cfg(test)]
mod tests {
    use Registers;
    #[test]
    fn getting_16_bit_from_b_and_c() {
        let regs = Registers {
            b: 0xF0,
            c: 0x0A,
            ..Default::default()
        };
        assert_eq!(regs.get_bc(), 0xF00A);
    }
}

fn main() {
    println!("Hello, world!");
}
