pub fn part1(input: &str) -> u64 {
    input
        .trim()
        .lines()
        .map(|banks| {
            joltage_max(banks.as_bytes(), 2)
        })
        .sum()
}

pub fn part2(input: &str) -> u64 {
    input
        .trim()
        .lines()
        .map(|banks| {
            joltage_max(banks.as_bytes(), 12)
        })
        .sum()
}

fn joltage_max(banks: &[u8], length: usize) -> u64 {
    let mut joltage = 0;
    let mut previous = 0;

    for i in (0..length).rev() {
        let (idx, battery) = find_max(&banks[previous..banks.len() - i]);
        previous += idx + 1;
        joltage = joltage * 10 + (battery - b'0') as u64;
    }

    joltage
}

fn find_max(banks: &[u8]) -> (usize, u8) {
    banks
        .iter()
        .copied()
        .enumerate()
        .rev()
        .max_by_key(|(_, battery)| *battery)
        .unwrap()
}
