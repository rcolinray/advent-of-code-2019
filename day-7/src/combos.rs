pub fn n_combos(n: i32) -> Vec<Vec<i32>> {
    let ns = (0..n).collect::<Vec<i32>>();
    combos(ns)
}

pub fn combos(ns: Vec<i32>) -> Vec<Vec<i32>> {
    if ns.len() == 1 {
        vec![ns.clone()]
    } else {
        let mut results = Vec::new();
        for i in 0..ns.len() {
            let mut new_ns = ns.clone();
            let first = new_ns.swap_remove(i as usize);
            new_ns.sort();
            let mut new_combos = combos(new_ns)
                .iter_mut()
                .map(|combo| {
                    let mut new_combo = vec![first];
                    new_combo.append(combo);
                    new_combo
                })
                .collect::<Vec<Vec<i32>>>();
            results.append(&mut new_combos);
        }
        results
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_n_combos1() {
        assert_eq!(n_combos(1), vec![vec![0]]);
    }

    #[test]
    fn test_n_combos2() {
        assert_eq!(n_combos(2), vec![vec![0, 1], vec![1, 0]]);
    }

    #[test]
    fn test_n_combos3() {
        assert_eq!(
            n_combos(3),
            vec![
                vec![0, 1, 2],
                vec![0, 2, 1],
                vec![1, 0, 2],
                vec![1, 2, 0],
                vec![2, 0, 1],
                vec![2, 1, 0]
            ]
        );
    }
}
