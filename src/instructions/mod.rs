struct CPU {}

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
mod tests {}
