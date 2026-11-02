use std::collections::HashMap;
use std::io;

// Input as [quota, weight1, weight2, ...]

fn shapley_shubik_values(input: &[u64]) -> Vec<f64> {
    let (&quota, weights) = input.split_first().unwrap();
    let voter_count = weights.len();

    let mut values: Vec<f64> = weights
        .iter()
        .enumerate()
        .map(|(pivot, &weight)| {
            let mut subsets = HashMap::from([((0usize, 0u64), 1.0)]);

            for (index, &other_weight) in weights.iter().enumerate() {
                if index == pivot {
                    continue;
                }

                let mut next = subsets.clone();
                for (&(size, sum), &count) in &subsets {
                    if sum + other_weight < quota {
                        *next.entry((size + 1, sum + other_weight)).or_insert(0.0) += count;
                    }
                }
                subsets = next;
            }

            let first_pivotal_sum = quota.saturating_sub(weight);
            let mut value = 0.0;
            let mut combinations = 1.0;

            for size in 0..voter_count {
                let pivotal_subsets: f64 = subsets
                    .iter()
                    .filter(|&(&(subset_size, sum), _)| {
                        subset_size == size && sum >= first_pivotal_sum
                    })
                    .map(|(_, count)| count)
                    .sum();

                value += pivotal_subsets / (voter_count as f64 * combinations);
                if size + 1 < voter_count {
                    combinations *= (voter_count - size - 1) as f64 / (size + 1) as f64;
                }
            }

            value
        })
        .collect();

    let total: f64 = values.iter().sum();
    for value in &mut values {
        *value /= total;
    }

    values
}

fn parse_input(input: &str) -> Vec<u64> {
    input
        .trim()
        .trim_matches(|character: char| character == '[' || character == ']')
        .split(|character: char| character == ',' || character.is_whitespace())
        .filter(|part| !part.is_empty())
        .map(|part| part.parse().unwrap())
        .collect()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let values = shapley_shubik_values(&parse_input(&input));
    let output = values
        .iter()
        .map(|value| value.to_string())
        .collect::<Vec<_>>()
        .join(", ");
    println!("[{output}]");
}
