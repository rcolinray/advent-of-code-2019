use std::env;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2);
    let filename = &args[1];

    let spacecraft = Spacecraft::from_file(filename);
    println!("fuel required: {:?}", spacecraft.fuel_required());
}

pub fn ideal_fuel_required(mass: f32) -> f32 {
    let fuel_mass = (mass / 3.0).floor() - 2.0;
    if fuel_mass > 0.0 {
        fuel_mass
    } else {
        0.0
    }
}

pub fn fuel_required(mass: f32) -> f32 {
    let mut fuel_mass = ideal_fuel_required(mass);
    let mut total_mass = fuel_mass;
    loop {
        let additional_mass = ideal_fuel_required(fuel_mass);
        if additional_mass <= 0.0 {
            break
        }
        total_mass += additional_mass;
        fuel_mass = additional_mass;
    }
    total_mass
}

pub struct Module {
    mass: f32,
}

impl Module {
    pub fn new(mass: f32) -> Module {
        Module {
            mass
        }
    }

    pub fn fuel_required(&self) -> f32 {
        fuel_required(self.mass)
    }
}

pub struct Spacecraft {
    modules: Vec<Module>,
}

impl Spacecraft {
    pub fn new(modules: Vec<Module>) -> Spacecraft {
        Spacecraft {
            modules,
        }
    }

    pub fn from_file(filename: &str) -> Spacecraft {
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
        let mut modules = Vec::new();
        for line in reader.lines() {
            let mass = line.unwrap().parse::<f32>().unwrap();
            let module = Module::new(mass);
            modules.push(module);
        }
        Spacecraft::new(modules)
    }

    pub fn fuel_required(&self) -> f32 {
        self.modules.iter().map(|module| module.fuel_required()).sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fuel_required() {
        assert_eq!(fuel_required(14.0), 2.0);
        assert_eq!(fuel_required(1969.0), 966.0);
    }

    // #[test]
    // fn test_module_fuel_required() {
    //     assert_eq!(Module::new(12.0).fuel_required(), 2.0);
    //     assert_eq!(Module::new(14.0).fuel_required(), 2.0);
    //     assert_eq!(Module::new(1969.0).fuel_required(), 654.0);
    //     assert_eq!(Module::new(100756.0).fuel_required(), 33583.0);
    // }

    // #[test]
    // fn test_spacecraft_fuel_required() {
    //     let modules = vec!(Module::new(12.0), Module::new(14.0), Module::new(1969.0), Module::new(100756.0));
    //     let spacecraft = Spacecraft::new(modules);
    //     assert_eq!(spacecraft.fuel_required(), 34241.0);
    // }
}