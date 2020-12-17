use super::solution::{Error as ChallengeErr, Solution};

struct BusInfo {
    id: u64,
    idx: u64,
}

fn parse_buses(buses_text: &str) -> Vec<BusInfo> {
    buses_text
        .split(',')
        .enumerate()
        .filter(|(_, el)| *el != "x")
        .map(|(idx, el)| BusInfo {
            id: el.parse().unwrap(),
            idx: idx as u64,
        })
        .collect()
}

fn parse_notes(notes_text: &str) -> (u64, Vec<BusInfo>) {
    let mut notes_lines = notes_text.lines();
    let timestamp: u64 = notes_lines.next().unwrap().parse().unwrap();

    (timestamp, parse_buses(notes_lines.next().unwrap()))
}

fn find_earliest_bus_estimation(timestamp: &u64, buses: &[BusInfo]) -> u64 {
    let (mut minutes_to_wait, mut earliest_bus) = (1_000_000_000 as u64, 0 as u64);

    for bus_info in buses.iter() {
        let bus_id = bus_info.id;

        let time_to_wait = (*timestamp as f64 / bus_id as f64).ceil() as u64 * bus_id - timestamp;

        if time_to_wait < minutes_to_wait {
            minutes_to_wait = time_to_wait;
            earliest_bus = bus_id;
        }
    }

    earliest_bus * minutes_to_wait
}

fn mod_pow(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    if modulus == 1 {
        return 0;
    }
    let mut result = 1;
    base %= modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = result * base % modulus;
        }
        exp >>= 1;
        base = base * base % modulus
    }
    result
}

fn mod_inverse(n: u64, m: u64) -> u64 {
    mod_pow(n, m - 2, m)
}

fn chinese_reminder_theorem_solver(r_i: &[u64], a_i: &[u64]) -> u64 {
    let modulo: u64 = a_i.iter().product();
    let mi: Vec<u64> = a_i.iter().map(|a| modulo / a).collect();
    let mi_inv: Vec<u64> = mi
        .iter()
        .zip(a_i.iter())
        .map(|(m, a)| mod_inverse(*m, *a))
        .collect();

    r_i.iter()
        .zip(mi.iter())
        .zip(mi_inv.iter())
        .map(|((&r, &m), &m_i)| r * m * m_i)
        .sum::<u64>()
        % modulo
}

fn find_gold_coin_timestamp(buses: &[BusInfo]) -> u64 {
    let r_i: Vec<u64> = buses.iter().map(|b| b.id - b.idx % b.id).collect();
    let a_i: Vec<u64> = buses.iter().map(|b| b.id).collect();

    chinese_reminder_theorem_solver(&r_i, &a_i)
}

pub struct Day13 {}

impl Solution for Day13 {
    fn first_task(&self, notes_text: &str) -> Result<String, ChallengeErr> {
        let (timestamp, buses) = parse_notes(&notes_text);

        Ok(find_earliest_bus_estimation(&timestamp, &buses).to_string())
    }

    fn second_task(&self, notes_text: &str) -> Result<String, ChallengeErr> {
        let (_, buses) = parse_notes(&notes_text);

        Ok(find_gold_coin_timestamp(&buses).to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_both_values() {
        let test_notes_1_text = "939
7,13,x,x,59,x,31,19";

        let (test_timestamp, test_buses) = parse_notes(test_notes_1_text);

        assert_eq!(
            find_earliest_bus_estimation(&test_timestamp, &test_buses),
            295
        );
        assert_eq!(find_gold_coin_timestamp(&test_buses), 1068781);
    }

    #[test]
    fn test_find_gold_coin_timestamp_extended() {
        let test_gold_timestamps = vec![
            ("17,x,13,19", 3417),
            ("67,7,59,61", 754018),
            ("67,x,7,59,61", 779210),
            ("67,7,x,59,61", 1261476),
            ("1789,37,47,1889", 1202161486),
        ];

        for (buses_text, gold_timestamp) in test_gold_timestamps {
            assert_eq!(
                find_gold_coin_timestamp(&parse_buses(buses_text)),
                gold_timestamp
            );
        }
    }
}
