use registers::Registers;

struct CPU {
    registers: Registers,
}

enum Instruction {
    ADD(ArithmeticTarget),
}

enum ArithmeticTarget {
    A,
    B,
    C,
    D,
    E,
    F,
    H,
    L,
}

impl CPU {
    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::ADD(target) => {
                match target {
                    ArithmeticTarget::C => {
                        let value = self.registers.c;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                    }
                    _ => { /* TODO: implement more targets */ }
                }
            }
            _ => { /* TODO: implement more instructions */ }
        }
    }

    fn add(&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = self.registers.a.overflowing_add(value);
        // TODO set flags
        new_value
    }
}

#[cfg(test)]
mod tests {
    use instructions::{ArithmeticTarget, Instruction, CPU};
    use registers::Registers;

    #[test]
    fn add_c_to_a() {
        let regs = Registers {
            a: 0,
            c: 12,
            ..Default::default()
        };
        let mut cpu = CPU { registers: regs };
        cpu.execute(Instruction::ADD(ArithmeticTarget::C));
        assert_eq!(cpu.registers.a, 12);
    }

    #[test]
    fn add_sets_the_flags_to_false() {
        let regs = Registers {
            a: 0,
            c: 12,
            ..Default::default()
        };
        let mut cpu = CPU { registers: regs };
        cpu.execute(Instruction::ADD(ArithmeticTarget::C));

        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.substract, false);
        assert_eq!(cpu.registers.f.half_carry, false);
        assert_eq!(cpu.registers.f.carry, false);
    }
}
