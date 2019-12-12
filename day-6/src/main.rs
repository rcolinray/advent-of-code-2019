mod lib;

use lib::load_puzzle_input;

fn main() {
    let system = load_puzzle_input("./input.txt");
    let total = system.total_orbit_count();
    println!("part 1: {}", total);

    let transfers = system.count_min_transfers("YOU", "SAN");
    println!("part 2: {}", transfers);
}
