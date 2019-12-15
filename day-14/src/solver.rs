use std::collections::{HashMap, VecDeque};

pub use crate::reaction::*;

const ORE: &str = "ORE";
const FUEL: &str = "FUEL";

pub struct Solver {
    pub show_your_work: bool,
    reactions: HashMap<String, Reaction>,
    ore_required: usize,
    required_compounds: VecDeque<Component>,
    leftover_amounts: HashMap<Compound, usize>,
}

impl Solver {
    pub fn init(puzzle: &Vec<Reaction>, fuel_amt: usize) -> Self {
        Solver::new(puzzle, fuel_amt, HashMap::new())
    }

    pub fn new(
        puzzle: &Vec<Reaction>,
        fuel_amt: usize,
        initial_amounts: HashMap<String, usize>,
    ) -> Self {
        let mut reactions = HashMap::new();
        for reaction in puzzle.iter() {
            let compound = reaction.get_output();
            assert!(!reactions.contains_key(compound));
            if reaction.has_input(ORE) {
                assert!(reaction.num_inputs() == 1);
            }
            reactions.insert(compound.clone(), reaction.clone());
        }

        let mut required_compounds = VecDeque::new();
        required_compounds.push_back(Component::new(FUEL, fuel_amt));

        Solver {
            show_your_work: false,
            reactions,
            ore_required: 0,
            required_compounds,
            leftover_amounts: initial_amounts,
        }
    }

    pub fn solve(&mut self) -> usize {
        while self.required_compounds.len() > 0 {
            self.step_solve();
        }

        if self.show_your_work {
            println!("\n---------------- DONE ----------------");
            println!("we require {} total ORE", self.ore_required);
        }

        self.ore_required
    }

    pub fn solve_with_leftovers(&mut self) -> (usize, HashMap<Compound, usize>) {
        (self.solve(), self.leftover_amounts.clone())
    }

    fn step_solve(&mut self) {
        if self.show_your_work {
            println!("\n---------------- SOLVING... ----------------");
        }
        let required_component = self
            .required_compounds
            .pop_front()
            .expect("Failed to get next required component");
        let compound = required_component.compound;
        let required_amt = required_component.amount;

        if self.show_your_work {
            println!("we require {} {}", required_amt, compound);
        }
        if compound == ORE {
            self.ore_required += required_amt;
            return;
        }

        let leftover_amt = self.get_leftovers(&compound);

        if self.show_your_work {
            println!("we have {} {} leftover", leftover_amt, compound);
        }
        if leftover_amt >= required_amt {
            if self.show_your_work {
                println!("using {} leftover {}", required_amt, compound);
            }
            self.set_leftovers(&compound, leftover_amt - required_amt);
            return;
        }

        let reaction = self
            .reactions
            .get(&compound)
            .expect(&format!("Failed to get reaction for compound {}", compound));

        let reaction_amt = reaction.get_amount();

        if self.show_your_work {
            println!("reaction for {0} produces {1} {0}", compound, reaction_amt);
        }
        let (multiplier, remainder) = self.calc_reaction(reaction, required_amt, leftover_amt);

        if self.show_your_work {
            println!(
                "run reaction {} time(s) with {} {} leftover",
                multiplier, remainder, compound
            );
        }

        let inputs = reaction.get_inputs();
        for component in inputs {
            let required = component.multiply_by(multiplier);
            if self.show_your_work {
                println!(
                    "we require {} {} to produce {} {}",
                    required.amount, required.compound, required_amt, compound
                );
            }
            self.required_compounds.push_back(required);
        }

        if remainder != leftover_amt {
            self.set_leftovers(&compound, remainder);
        }

        if self.show_your_work {
            self.log_leftovers();
        }
    }

    fn calc_reaction(
        &self,
        reaction: &Reaction,
        required_amt: usize,
        leftover_amt: usize,
    ) -> (usize, usize) {
        let reaction_amt = reaction.get_amount();
        if reaction_amt >= required_amt {
            return (1, leftover_amt + reaction_amt - required_amt);
        }

        let multiplier = required_amt / reaction_amt;
        let mut final_amt = reaction_amt * multiplier;
        if final_amt >= required_amt {
            let remainder = final_amt % required_amt;
            (multiplier, leftover_amt + remainder)
        } else if final_amt + leftover_amt >= required_amt {
            final_amt += leftover_amt;
            let remainder = final_amt - required_amt;
            (multiplier, remainder)
        } else {
            let multiplier = (required_amt / reaction_amt) + 1;
            let remainder = reaction_amt * multiplier - required_amt;
            (multiplier, leftover_amt + remainder)
        }
    }

    fn get_leftovers(&self, compound: &str) -> usize {
        *self.leftover_amounts.get(compound).unwrap_or(&0)
    }

    fn set_leftovers(&mut self, compound: &str, amount: usize) {
        if amount == 0 {
            self.leftover_amounts.remove(compound);
        } else {
            self.leftover_amounts.insert(compound.to_owned(), amount);
        }
    }

    fn log_leftovers(&self) {
        for (compound, amount) in self.leftover_amounts.iter() {
            println!("we have {} leftover {}", amount, compound);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solver;
    use crate::puzzle::parse_puzzle_input;

    #[test]
    fn test_solve_example1() {
        let input = "10 ORE => 10 A
                     1 ORE => 1 B
                     7 A, 1 B => 1 C
                     7 A, 1 C => 1 D
                     7 A, 1 D => 1 E
                     7 A, 1 E => 1 FUEL";
        let reactions = parse_puzzle_input(input.as_bytes());
        assert_eq!(reactions.len(), 6);
        let mut solver = Solver::init(&reactions, 1);
        assert_eq!(solver.solve(), 31);
    }

    #[test]
    fn test_solve_example2() {
        let input = "9 ORE => 2 A
                     8 ORE => 3 B
                     7 ORE => 5 C
                     3 A, 4 B => 1 AB
                     5 B, 7 C => 1 BC
                     4 C, 1 A => 1 CA
                     2 AB, 3 BC, 4 CA => 1 FUEL";
        let reactions = parse_puzzle_input(input.as_bytes());
        assert_eq!(reactions.len(), 7);
        let mut solver = Solver::init(&reactions, 1);
        assert_eq!(solver.solve(), 165);
    }

    #[test]
    fn test_solve_example3() {
        let input = "157 ORE => 5 NZVS
                     165 ORE => 6 DCFZ
                     44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
                     12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
                     179 ORE => 7 PSHF
                     177 ORE => 5 HKGWZ
                     7 DCFZ, 7 PSHF => 2 XJWVT
                     165 ORE => 2 GPVTF
                     3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT";
        let reactions = parse_puzzle_input(input.as_bytes());
        assert_eq!(reactions.len(), 9);
        let mut solver = Solver::init(&reactions, 1);
        assert_eq!(solver.solve(), 13312);
    }

    #[test]
    fn test_solve_example4() {
        let input = "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
                     17 NVRVD, 3 JNWZP => 8 VPVL
                     53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
                     22 VJHF, 37 MNCFX => 5 FWMGM
                     139 ORE => 4 NVRVD
                     144 ORE => 7 JNWZP
                     5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
                     5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
                     145 ORE => 6 MNCFX
                     1 NVRVD => 8 CXFTF
                     1 VJHF, 6 MNCFX => 4 RFSQX
                     176 ORE => 6 VJHF";
        let reactions = parse_puzzle_input(input.as_bytes());
        assert_eq!(reactions.len(), 12);
        let mut solver = Solver::init(&reactions, 1);
        assert_eq!(solver.solve(), 180697);
    }

    #[test]
    fn test_solve_example5() {
        let input = "171 ORE => 8 CNZTR
                     7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
                     114 ORE => 4 BHXH
                     14 VRPVC => 6 BMBT
                     6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
                     6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
                     15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
                     13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
                     5 BMBT => 4 WPTQ
                     189 ORE => 9 KTJDG
                     1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
                     12 VRPVC, 27 CNZTR => 2 XDBXC
                     15 KTJDG, 12 BHXH => 5 XCVML
                     3 BHXH, 2 VRPVC => 7 MZWV
                     121 ORE => 7 VRPVC
                     7 XCVML => 6 RJRHP
                     5 BHXH, 4 VRPVC => 5 LTCX";
        let reactions = parse_puzzle_input(input.as_bytes());
        assert_eq!(reactions.len(), 17);
        let mut solver = Solver::init(&reactions, 1);
        assert_eq!(solver.solve(), 2210736);
    }
}
