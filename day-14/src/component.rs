pub type Compound = String;

#[derive(Debug, Clone, PartialEq)]
pub struct Component {
    pub compound: Compound,
    pub amount: usize,
}

impl Component {
    pub fn new(compound: &str, amount: usize) -> Self {
        Component {
            compound: compound.to_owned(),
            amount,
        }
    }

    pub fn from_string(string: &str) -> Option<Component> {
        match &string.trim().split(' ').collect::<Vec<_>>()[..] {
            &[amount, compound] => Some((amount, compound)),
            _ => None,
        }
        .and_then(|(amount, compound)| {
            amount
                .parse::<usize>()
                .ok()
                .map(|amount| (amount, compound))
        })
        .map(|(amount, compound)| Component::new(compound, amount))
    }

    pub fn multiply_by(&self, multiplier: usize) -> Self {
        Component::new(&self.compound, self.amount * multiplier)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_component_from_string_good() {
        assert_eq!(
            Component::from_string("42 HELLO"),
            Some(Component::new("HELLO", 42))
        );
    }

    #[test]
    fn test_component_from_string_bad() {
        assert_eq!(Component::from_string("OOPS 13"), None);
    }
}
