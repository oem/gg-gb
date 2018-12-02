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
                        // TODO: implement ADD on register C
                    }
                    _ => { /* TODO: implement more targets */ }
                }
            }
            _ => { /* TODO: implement more instructions */ }
        }
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
}
