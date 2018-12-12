use registers::Registers;

struct CPU {
    registers: Registers,
    pc: u16,
    bus: MemoryBus,
}

struct MemoryBus {
    memory: [u8; 0xFFFF],
}

impl MemoryBus {
    fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }
}

enum Instruction {
    ADD(ArithmeticTarget),
}

impl Instruction {
    fn from_byte(_b: u8) -> Option<Instruction> {
        None
    }
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
    fn execute(&mut self, instruction: Instruction) -> u16 {
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
        0
    }

    fn step(&mut self) {
        let mut instruction_byte = self.bus.read_byte(self.pc);

        let next_pc = if let Some(instruction) = Instruction::from_byte(instruction_byte) {
            self.execute(instruction)
        } else {
            panic!("Unknown instruction found for : 0x{:x}", instruction_byte);
        };

        self.pc = next_pc;
    }

    fn add(&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = self.registers.a.overflowing_add(value);
        self.registers.f.zero = new_value == 0;
        self.registers.f.substract = false;
        self.registers.f.carry = did_overflow;
        self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;
        new_value
    }
}

#[cfg(test)]
mod tests {
    use instructions::{ArithmeticTarget, Instruction, MemoryBus, CPU};
    use registers::Registers;

    #[test]
    fn add_c_to_a() {
        let mut cpu = CPU {
            registers: Registers {
                a: 0,
                c: 12,
                ..Default::default()
            },
            pc: 0,
            bus: MemoryBus {
                memory: [0; 0xFFFF],
            },
        };
        cpu.execute(Instruction::ADD(ArithmeticTarget::C));
        assert_eq!(cpu.registers.a, 12);
    }

    #[test]
    fn add_sets_the_flags_to_false() {
        let mut cpu = CPU {
            registers: Registers {
                a: 0,
                c: 12,
                ..Default::default()
            },
            pc: 0,
            bus: MemoryBus {
                memory: [0; 0xFFFF],
            },
        };
        cpu.execute(Instruction::ADD(ArithmeticTarget::C));

        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.substract, false);
        assert_eq!(cpu.registers.f.half_carry, false);
        assert_eq!(cpu.registers.f.carry, false);
    }

    #[test]
    fn add_with_nibble_overflow_sets_half_carry() {
        let mut cpu = CPU {
            registers: Registers {
                a: 0b00000001,
                c: 0b00001111,
                ..Default::default()
            },
            pc: 0,
            bus: MemoryBus {
                memory: [0; 0xFFFF],
            },
        };
        cpu.execute(Instruction::ADD(ArithmeticTarget::C));

        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.substract, false);
        assert_eq!(cpu.registers.f.half_carry, true);
        assert_eq!(cpu.registers.f.carry, false);
    }

    #[test]
    fn add_with_overflow_sets_carry() {
        let mut cpu = CPU {
            registers: Registers {
                a: 0b11111111,
                c: 0b00000001,
                ..Default::default()
            },
            pc: 0,
            bus: MemoryBus {
                memory: [0; 0xFFFF],
            },
        };
        cpu.execute(Instruction::ADD(ArithmeticTarget::C));

        assert_eq!(cpu.registers.f.carry, true);
    }
}
