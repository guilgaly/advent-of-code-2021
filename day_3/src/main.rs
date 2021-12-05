use bitvec::prelude::*;

static INPUT: &str = include_str!("input");

#[derive(Debug, PartialEq)]
struct Rates {
    gamma: usize,
    epsilon: usize,
    oxygen: usize,
    co2: usize,
}

fn main() -> Result<(), String> {
    let diagnostics = parse_input(INPUT)?;
    let rates = calculate_rates(&diagnostics);
    println!("Part 1 result: {}", rates.gamma * rates.epsilon);
    println!("Part 2 result: {}", rates.oxygen * rates.co2);
    Ok(())
}

fn parse_input(input: &str) -> Result<Vec<BitVec>, String> {
    input
        .lines()
        .map(|line| {
            // let mut report_line = BitVec::
            line.chars()
                .map(|char| match char {
                    '0' => Ok(false),
                    '1' => Ok(true),
                    _ => Err(format!("Invalid bit value {}", char)),
                })
                .collect::<Result<BitVec, String>>()
        })
        .collect()
}

fn calculate_rates(diagnostics: &[BitVec]) -> Rates {
    let diags_count = diagnostics.len();
    let true_bits_counts = count_true_bits(diagnostics);

    fn calculate_rate<C>(true_bits_counts: &[usize], criteria: C) -> BitVec
    where
        C: Fn(&usize) -> bool,
    {
        true_bits_counts.iter().map(criteria).collect()
    }
    let gamma_bits = calculate_rate(&true_bits_counts, |count| *count > (diags_count / 2));
    let epsilon_bits = calculate_rate(&true_bits_counts, |count| *count < (diags_count / 2));

    fn calculate_rating<C>(diagnostics: &[BitVec], criteria: C) -> BitVec
    where
        C: Fn(usize, usize) -> bool,
    {
        let mut possible_ratings = Vec::from(diagnostics);
        let mut bit_idx = 0;
        while possible_ratings.len() > 1 {
            let true_bits_counts = count_true_bits(&possible_ratings);
            let expected_bit = criteria(true_bits_counts[bit_idx], possible_ratings.len());
            possible_ratings.retain(|diagnostic| diagnostic[bit_idx] == expected_bit);
            bit_idx += 1;
        }
        possible_ratings[0].clone()
    }
    let oxygen_rating = calculate_rating(&diagnostics, |count, total| {
        let expected = count * 2 >= total;
        println!("[Oxygen] count={}, total={}, expected={}", count, total, expected);
        expected
    });
    let co2_rating = calculate_rating(&diagnostics, |count, total| {
        let expected = count * 2 < total;
        println!("[CO2] count={}, total={}, expected={}", count, total, expected);
        expected
    });

    Rates {
        gamma: bits_to_usize(&gamma_bits),
        epsilon: bits_to_usize(&epsilon_bits),
        oxygen: bits_to_usize(&oxygen_rating),
        co2: bits_to_usize(&co2_rating),
    }
}

fn count_true_bits(diagnostics: &[BitVec]) -> Vec<usize> {
    let diag_len = diagnostics[0].len();
    let mut true_bits_counts: Vec<usize> = vec![0; diag_len];
    for diagnostic in diagnostics {
        for x in 0..diag_len {
            if diagnostic[x] {
                true_bits_counts[x] += 1
            }
        }
    }
    true_bits_counts
}

fn bits_to_usize(bits: &BitSlice) -> usize {
    let mut v = 0;
    for i in 0..bits.len() {
        if bits[i] {
            v |= 1 << (bits.len() - 1 - i);
        }
    }
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_input() -> [BitVec; 12] {
        [
            bitvec![0, 0, 1, 0, 0],
            bitvec![1, 1, 1, 1, 0],
            bitvec![1, 0, 1, 1, 0],
            bitvec![1, 0, 1, 1, 1],
            bitvec![1, 0, 1, 0, 1],
            bitvec![0, 1, 1, 1, 1],
            bitvec![0, 0, 1, 1, 1],
            bitvec![1, 1, 1, 0, 0],
            bitvec![1, 0, 0, 0, 0],
            bitvec![1, 1, 0, 0, 1],
            bitvec![0, 0, 0, 1, 0],
            bitvec![0, 1, 0, 1, 0],
        ]
    }

    #[test]
    fn test_calculate_rates() {
        let actual = calculate_rates(&test_input());
        let expected = Rates {
            gamma: 22,
            epsilon: 9,
            oxygen: 23,
            co2: 10,
        };
        assert_eq!(expected, actual);
    }
}
