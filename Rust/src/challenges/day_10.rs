use super::challenge::{Challenge, Error as ChallengeErr};

fn parse_adapters(adapters_text: &str) -> Vec<u64> {
    adapters_text
        .lines()
        .map(|el| el.parse().unwrap())
        .collect()
}

fn prepare_devices(adapters: &[u64]) -> Vec<u64> {
    let mut cloned = adapters.to_owned();
    cloned.push(0);
    cloned.push(*adapters.iter().max().unwrap() + 3);

    cloned.sort_unstable();

    cloned
}

fn find_jolt_differences(adapters: &[u64]) -> (u64, u64, u64, u64) {
    let devices = prepare_devices(adapters);
    let mut diffs = [0, 0, 0, 0];

    for i in 1..devices.len() {
        diffs[(devices[i] - devices[i - 1]) as usize] += 1
    }

    (diffs[0], diffs[1], diffs[2], diffs[3])
}

fn find_product_of_jolt_differences(adapters: &[u64]) -> u64 {
    let (_, ones, _, threes) = find_jolt_differences(adapters);
    ones * threes
}

fn count_number_of_ways_to_connect(adapters: &[u64]) -> u64 {
    let devices = prepare_devices(adapters);
    let mut ways = vec![0; devices.len()];
    ways[0] = 1;

    for i in 1..devices.len() {
        let lower_limit = if i < 4 { 0 } else { i - 4 };
        for j in (lower_limit..i).rev() {
            if devices[i] - devices[j] < 4 {
                ways[i] += ways[j];
            }
        }
    }

    ways[ways.len() - 1]
}

pub struct Solution {}

impl Challenge for Solution {
    fn first_part(&self, input: &str) -> Result<String, ChallengeErr> {
        let adapters = parse_adapters(&input);

        Ok(find_product_of_jolt_differences(&adapters).to_string())
    }

    fn second_part(&self, input: &str) -> Result<String, ChallengeErr> {
        let adapters = parse_adapters(&input);

        Ok(count_number_of_ways_to_connect(&adapters).to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let test_adapters_1_text = "16
10
15
5
1
11
7
19
6
12
4";

        let test_adapters_1 = parse_adapters(&test_adapters_1_text);
        assert_eq!(find_jolt_differences(&test_adapters_1), (0, 7, 0, 5));
        assert_eq!(count_number_of_ways_to_connect(&test_adapters_1), 8);
    }

    #[test]
    fn test_example_2() {
        let test_adapters_2_text = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";

        let test_adapters_2 = parse_adapters(&test_adapters_2_text);
        assert_eq!(find_jolt_differences(&test_adapters_2), (0, 22, 0, 10));
        assert_eq!(count_number_of_ways_to_connect(&test_adapters_2), 19208);
    }
}
