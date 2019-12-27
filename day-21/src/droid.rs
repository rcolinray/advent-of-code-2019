#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Register {
    T,
    J,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
}

impl Register {
    fn from_string(text: &str) -> Option<Self> {
        match text {
            "T" => Some(Self::T),
            "J" => Some(Self::J),
            "A" => Some(Self::A),
            "B" => Some(Self::B),
            "C" => Some(Self::C),
            "D" => Some(Self::D),
            "E" => Some(Self::E),
            "F" => Some(Self::F),
            "G" => Some(Self::G),
            "H" => Some(Self::H),
            "I" => Some(Self::I),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Instruction {
    And(Register, Register),
    Or(Register, Register),
    Not(Register, Register),
}

impl Instruction {
    fn from_string(text: &str) -> Option<Self> {
        let parts = text.trim().split(' ').collect::<Vec<_>>();
        if parts.len() != 3 {
            return None;
        }

        if let Some((reg1, reg2)) = Register::from_string(parts[1])
            .and_then(|reg1| Register::from_string(parts[2]).map(|reg2| (reg1, reg2)))
        {
            match parts[0] {
                "AND" => Some(Self::And(reg1, reg2)),
                "OR" => Some(Self::Or(reg1, reg2)),
                "NOT" => Some(Self::Not(reg1, reg2)),
                _ => None,
            }
        } else {
            None
        }
    }
}

fn parse_script(script: &str) -> Vec<Instruction> {
    script
        .lines()
        .map(|line| Instruction::from_string(line).expect("Failed to parse instruction"))
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::Instruction::*;
    use super::Register::*;
    use super::*;

    #[test]
    fn test_parse_script() {
        let ins = parse_script(
            "NOT A J
             NOT B T
             OR T J
             NOT C T
             OR T J
             AND D J",
        );
        assert_eq!(
            ins,
            vec![
                Not(A, J),
                Not(B, T),
                Or(T, J),
                Not(C, T),
                Or(T, J),
                And(D, J)
            ]
        )
    }
}
