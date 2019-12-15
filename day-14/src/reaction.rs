use std::collections::HashMap;

pub use crate::component::*;

#[derive(Debug, Clone)]
pub struct Reaction {
    inputs: HashMap<Compound, usize>,
    output: Compound,
    amount: usize,
}

impl Reaction {
    pub fn new(inputs: Vec<Component>, output: Component) -> Self {
        let mut input_map = HashMap::new();
        for component in inputs.into_iter() {
            input_map.insert(component.compound, component.amount);
        }
        Reaction {
            inputs: input_map,
            output: output.compound,
            amount: output.amount,
        }
    }

    pub fn from_string(string: &str) -> Option<Self> {
        match &string.split("=>").collect::<Vec<_>>()[..] {
            &[input, output] => Some((input, output)),
            _ => None,
        }
        .and_then(|(input, output)| {
            let mut inputs = Vec::new();
            for input in input.split(',').map(|input| Component::from_string(input)) {
                if let Some(component) = input {
                    inputs.push(component);
                } else {
                    return None;
                }
            }

            Component::from_string(output).map(|output| Reaction::new(inputs, output))
        })
    }

    pub fn has_input(&self, input: &str) -> bool {
        self.get_input(input).is_some()
    }

    pub fn get_input(&self, input: &str) -> Option<usize> {
        self.inputs.get(input).copied()
    }

    pub fn num_inputs(&self) -> usize {
        self.inputs.len()
    }

    pub fn get_inputs(&self) -> Vec<Component> {
        self.inputs
            .iter()
            .map(|(compound, &amount)| Component::new(compound, amount))
            .collect::<Vec<_>>()
    }

    pub fn get_output(&self) -> &Compound {
        &self.output
    }

    pub fn get_amount(&self) -> usize {
        self.amount
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reaction_from_string_good() {
        let reaction = Reaction::from_string("1 JNDQ, 11 PHNC => 7 LBJSB");
        assert_eq!(reaction.is_some(), true);

        let reaction = reaction.unwrap();
        assert_eq!(reaction.get_input("JNDQ"), Some(1));
        assert_eq!(reaction.get_input("PHNC"), Some(11));
        assert_eq!(reaction.get_input("LBJSB"), None);

        let output = reaction.get_output();
        let amount = reaction.get_amount();
        assert_eq!(output, "LBJSB");
        assert_eq!(amount, 7);
    }

    #[test]
    fn test_reaction_from_string_bad() {
        let reaction = Reaction::from_string("1 JNDQ, PHNC 11 => 7 LBJSB");
        assert_eq!(reaction.is_none(), true);
    }
}
