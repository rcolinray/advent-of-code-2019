const BASE_PATTERN: &[i32; 4] = &[0, 1, 0, -1];

pub fn fft(off_idx: usize, out_idx: usize, data: &Vec<i32>) -> i32 {
    let eff_out_idx = off_idx + out_idx;
    let sum = &data[out_idx..]
        .iter()
        .enumerate()
        .map(|(i, &x)| {
            let in_idx = eff_out_idx + i;
            let coeff = get_coeff(eff_out_idx, in_idx);
            coeff * x
        })
        .sum::<i32>();
    (sum % 10).abs()
}

#[inline(always)]
fn get_coeff(out_idx: usize, in_idx: usize) -> i32 {
    let patt_idx = ((in_idx + 1) / (out_idx + 1)) % 4;
    BASE_PATTERN[patt_idx]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_coeffs_for_phase(phase: usize, n: usize) -> Vec<i32> {
        (0..n).map(|i| get_coeff(phase, i)).collect::<Vec<i32>>()
    }

    #[test]
    fn test_get_coeff() {
        let coeffs = get_coeffs_for_phase(0, 8);
        assert_eq!(coeffs, vec![1, 0, -1, 0, 1, 0, -1, 0]);

        let coeffs = get_coeffs_for_phase(1, 8);
        assert_eq!(coeffs, vec![0, 1, 1, 0, 0, -1, -1, 0]);

        let coeffs = get_coeffs_for_phase(2, 8);
        assert_eq!(coeffs, vec![0, 0, 1, 1, 1, 0, 0, 0]);
    }
}
