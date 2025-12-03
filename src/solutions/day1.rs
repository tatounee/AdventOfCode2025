pub fn part1(input: &str) -> u32 {
    let mut pos = 50;

    input
        .trim()
        .lines()
        .filter(|act| {
            let delta = act[1..].parse::<i32>().unwrap();
            match act.as_bytes()[0] {
                b'R' => pos = (pos + delta).rem_euclid(100),
                _ => pos = (pos - delta).rem_euclid(100),
            }

            pos == 0
        })
        .count() as u32
}

pub fn part2(input: &str) -> i32 {
    let mut pos = 50;

    input
        .trim()
        .lines()
        .map(|act| {
            let mut delta = act[1..].parse::<i32>().unwrap();
            // delta = delta * (act.as_bytes()[0] as i32) * 2 - 1;

            let mut pass0 = delta / 100;
            delta %= 100;

            if delta == 0 {
                return pass0;
            }

            let old = pos;
            match act.as_bytes()[0] {
                b'R' => pos = pos + delta,
                _ => pos = pos - delta,
            }

            if old == 0 {
                pos = pos.rem_euclid(100);
                return pass0;
            }

            if pos < 0 {
                pos += 100;
                pass0 += 1;
            } else if pos == 0 {
                pass0 += 1;
            } else if pos >= 100 {
                pos -= 100;
                pass0 += 1;
            }

            pass0
        })
        .sum::<i32>()
}
