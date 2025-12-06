pub fn part1(input: &str) -> u64 {
    let mut input = input.trim().lines().rev();

    let operations = input.next().unwrap();

    let tables: Vec<Vec<u64>> = input
        .map(|line| {
            line.trim()
                .split_ascii_whitespace()
                .map(|x| x.parse::<u64>().unwrap())
                .collect()
        })
        .collect();

    operations
        .trim()
        .split_ascii_whitespace()
        .enumerate()
        .map(|(j, op)| {
            tables
                .iter()
                .map(|line| line[j])
                .reduce(|acc, x| match op {
                    "*" => acc * x,
                    _ => acc + x,
                })
                .unwrap()
        })
        .sum()
}

pub fn part2(input: &str) -> u64 {
    let mut input = input.trim().lines().rev();

    let mut operations = input
        .next()
        .unwrap()
        .chars()
        .enumerate()
        .filter(|(_, c)| *c != ' ')
        .peekable();

    let tables: Vec<&str> = input.rev().collect();

    let mut total = 0;
    while let Some((i, op)) = operations.next() {
        let numbers: Vec<&str> = if let Some((j, _)) = operations.peek() {
            // Can be optimize since we know there will be 4 elements
            tables.iter().map(|line| &line[i..j - 1]).collect()
        } else {
            tables.iter().map(|line| &line[i..line.len()]).collect()
        };


        let problem = (0..numbers[0].len())
            .map(move |k| {
                numbers
                    .iter()
                    .map(|number| number.as_bytes()[k])
                    .filter(|c| *c != b' ')
                    .fold(0, |acc, c| acc * 10 + (c - b'0') as u64)
            })
            .reduce(|acc, x| match op {
                '*' => acc * x,
                _ => acc + x,
            })
            .unwrap();

        total += problem;
    }

    total
}
