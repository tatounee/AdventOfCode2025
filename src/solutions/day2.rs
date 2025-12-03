pub fn part1(input: &str) -> u64 {
    solution(input, |id| {
        let len = id.len();
        len % 2 == 0 && &id[..len / 2] == &id[len / 2..]
    })
}

pub fn part2(input: &str) -> u64 {
    solution(input, |id| {
        let len = id.len();

        for i in 1..=len / 2 {
            if len % i == 0 {
                let pat = &id[..i];
                let mut invalide = true;
                for j in 0..len / i {
                    let offset = j * i;
                    invalide &= pat == &id[offset..offset + i];
                }
                if invalide {
                    return true;
                }
            }
        }

        false
    })
}

fn solution(input: &str, is_invalide: impl Fn(&str) -> bool) -> u64 {
    input
        .trim()
        .split(',')
        .map(|range| {
            let (start, end) = range.split_once('-').unwrap();

            let start = start.parse::<u64>().unwrap();
            let end = end.parse::<u64>().unwrap();

            (start..=end).filter_map(|id| {
                let text = id.to_string();
                if is_invalide(&text) {
                    Some(id)
                } else {
                    None
                }
            })
        })
        .flatten()
        .sum()
}
