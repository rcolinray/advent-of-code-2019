mod mat4x3;

use mat4x3::Mat4x3;

use std::collections::HashMap;
use std::env;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let filename = if args.len() > 1 {
        &args[1]
    } else {
        "./input.txt"
    };
    part1(filename);
    part2(filename);
}

#[inline(always)]
fn sgn(x: i32) -> i32 {
    if x > 0 {
        1
    } else if x < 0 {
        -1
    } else {
        0
    }
}

struct System {
    components: [ComponentSystem; 3],
}

impl System {
    fn new(pos: Mat4x3) -> Self {
        System {
            components: [
                ComponentSystem::new(pos.axis(0)),
                ComponentSystem::new(pos.axis(1)),
                ComponentSystem::new(pos.axis(2)),
            ],
        }
    }

    fn simulate_step(&mut self) {
        for component in self.components.iter_mut() {
            component.simulate_step();
        }
    }

    fn get_pos(&self) -> Mat4x3 {
        Mat4x3::from_cols(
            self.components[0].pos,
            self.components[1].pos,
            self.components[2].pos,
        )
    }

    fn get_vel(&self) -> Mat4x3 {
        Mat4x3::from_cols(
            self.components[0].vel,
            self.components[1].vel,
            self.components[2].vel,
        )
    }

    fn get_total_energy(&self) -> i32 {
        let [p1, p2, p3, p4] = self.get_pos().sum_rows();
        let [k1, k2, k3, k4] = self.get_vel().sum_rows();
        (p1 * k1) + (p2 * k2) + (p3 * k3) + (p4 * k4)
    }

    fn is_initial_state(&self) -> bool {
        self.components.iter().all(|c| c.is_initial_state())
    }
}

fn part1(filename: &str) {
    let position = Mat4x3::from_file(filename);
    let mut system = System::new(position);
    for _ in 0..1000 {
        system.simulate_step();
    }
    println!("part 1: {}", system.get_total_energy());
}

struct ComponentSystem {
    init: [i32; 4],
    pos: [i32; 4],
    vel: [i32; 4],
}

impl ComponentSystem {
    fn new(pos: [i32; 4]) -> Self {
        ComponentSystem {
            init: pos,
            pos: pos,
            vel: [0, 0, 0, 0],
        }
    }

    fn is_initial_state(&self) -> bool {
        self.pos == self.init && self.vel == [0, 0, 0, 0]
    }

    fn simulate_step(&mut self) {
        self.vel[0] += sgn(self.pos[1] - self.pos[0])
            + sgn(self.pos[2] - self.pos[0])
            + sgn(self.pos[3] - self.pos[0]);
        self.vel[1] += sgn(self.pos[0] - self.pos[1])
            + sgn(self.pos[2] - self.pos[1])
            + sgn(self.pos[3] - self.pos[1]);
        self.vel[2] += sgn(self.pos[0] - self.pos[2])
            + sgn(self.pos[1] - self.pos[2])
            + sgn(self.pos[3] - self.pos[2]);
        self.vel[3] += sgn(self.pos[0] - self.pos[3])
            + sgn(self.pos[1] - self.pos[3])
            + sgn(self.pos[2] - self.pos[3]);

        self.pos[0] += self.vel[0];
        self.pos[1] += self.vel[1];
        self.pos[2] += self.vel[2];
        self.pos[3] += self.vel[3];
    }
}

fn is_prime(n: u128) -> bool {
    if n <= 3 {
        n > 1
    } else if n % 2 == 0 || n % 3 == 0 {
        false
    } else {
        let mut i: u128 = 5;
        while i.pow(2) <= n {
            if n % i == 0 || n % (i + 2) == 0 {
                return false;
            }
            i += 6;
        }
        true
    }
}

fn prime_factors(n: u128) -> Vec<u128> {
    inner_prime_factors(n, 2)
}

fn inner_prime_factors(n: u128, i: u128) -> Vec<u128> {
    let mut factors = Vec::new();
    for m in i..=n {
        if is_prime(m) && n % m == 0 {
            factors.push(m);
            factors.append(&mut inner_prime_factors(n / m, m));
            break;
        }
    }
    factors
}

fn occurrences(nums: Vec<u128>) -> HashMap<u128, u32> {
    let mut counts = HashMap::new();
    for p in nums {
        let count = counts.get(&p).unwrap_or(&0) + 1;
        counts.insert(p, count);
    }
    counts
}

fn lcm(nums: &[u128]) -> u128 {
    let mut primes = HashMap::new();
    for &n in nums {
        let counts = occurrences(prime_factors(n));
        for (&p, count) in counts.iter() {
            let new_count = *primes.get(&p).unwrap_or(&0).max(count);
            primes.insert(p, new_count);
        }
    }
    primes.iter().map(|(p, &e)| p.pow(e)).product()
}

fn part2(filename: &str) {
    let position = Mat4x3::from_file(filename);
    let mut iters: [u128; 3] = [0, 0, 0];

    for a in 0..3 {
        let initial_pos = position.axis(a);
        let mut system = ComponentSystem::new(initial_pos);
        let mut i: u128 = 0;

        loop {
            system.simulate_step();
            i += 1;
            if system.is_initial_state() {
                break;
            }
        }
        println!("found axis {}: {}", a, i);
        iters[a] = i;
    }

    println!("part 2: {}", lcm(&iters));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primality() {
        assert_eq!(is_prime(1), false);
        assert_eq!(is_prime(2), true);
        assert_eq!(is_prime(3), true);
        assert_eq!(is_prime(4), false);
        assert_eq!(is_prime(5), true);
        assert_eq!(is_prime(6), false);
        assert_eq!(is_prime(7), true);
        assert_eq!(is_prime(8), false);
        assert_eq!(is_prime(9), false);
        assert_eq!(is_prime(10), false);
    }

    #[test]
    fn test_prime_factors() {
        assert_eq!(prime_factors(1), vec![]);
        assert_eq!(prime_factors(2), vec![2]);
        assert_eq!(prime_factors(6), vec![2, 3]);
        assert_eq!(prime_factors(10), vec![2, 5]);
        assert_eq!(prime_factors(15), vec![3, 5]);
        assert_eq!(prime_factors(90), vec![2, 3, 3, 5]);
    }

    #[test]
    fn test_lcm() {
        assert_eq!(lcm(&[8, 9, 21]), 504);
        assert_eq!(lcm(&[18, 28, 44]), 2772);
        assert_eq!(lcm(&[2028, 5898, 4702]), 4686774924);
    }

    #[test]
    fn test_simulate_step() {
        let mut system = System::new(Mat4x3::new([
            [-1, 0, 2],
            [2, -10, -7],
            [4, -8, 8],
            [3, 5, -1],
        ]));
        system.simulate_step();

        assert_eq!(
            system.get_pos(),
            Mat4x3::new([[2, -1, 1], [3, -7, -4], [1, -7, 5], [2, 2, 0]])
        );

        assert_eq!(
            system.get_vel(),
            Mat4x3::new([[3, -1, -1], [1, 3, 3], [-3, 1, -3], [-1, -3, 1]])
        );

        for _ in 0..9 {
            system.simulate_step();
        }

        assert_eq!(
            system.get_pos(),
            Mat4x3::new([[2, 1, -3], [1, -8, 0], [3, -6, 1], [2, 0, 4]])
        );

        assert_eq!(
            system.get_vel(),
            Mat4x3::new([[-3, -2, 1], [-1, 1, 3], [3, 2, -3], [1, -1, -1]])
        );

        assert_eq!(system.get_total_energy(), 179);
    }

    #[test]
    fn test_part2_1() {
        let initial_pos = Mat4x3::new([[-1, 0, 2], [2, -10, -7], [4, -8, 8], [3, 5, -1]]);
        let mut system = System::new(initial_pos);
        for _ in 0..2772 {
            system.simulate_step();
        }
        assert_eq!(system.is_initial_state(), true);
    }
}
