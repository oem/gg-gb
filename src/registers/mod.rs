mod flags_register;
mod registers;

pub use registers::registers::Registers;

#[cfg(test)]
mod tests {
    use registers::flags_register::FlagsRegister;
    use registers::Registers;

    #[test]
    fn using_flags_register() {
        let regs = Registers {
            f: FlagsRegister {
                zero: true,
                ..Default::default()
            },
            ..Default::default()
        };
        assert_eq!(u8::from(regs.f), 0b1000_0000);
    }
}
