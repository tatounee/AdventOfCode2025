pub fn part1(input: &str) -> u32 {
    let map = Map::new(input);

    (0..map.map.len())
        .map(|y| {
            (0..map.map[y].len())
                .filter(|x| {
                    map.map[y][*x] == b'@' && map.count_neighbour(*x as isize, y as isize) < 4
                })
                .count() as u32
        })
        .sum()
}

pub fn part2(input: &str) -> u32 {
    let mut map1 = Map::new(input);
    let mut map2 = Map::new(input);

    let take_rolls = |buf1: &mut Map, buf2: &mut Map| {
        (0..buf1.map.len())
            .map(|y| {
                (0..buf1.map[y].len())
                    .filter(|x| {
                        if buf1.map[y][*x] == b'@'
                            && buf1.count_neighbour(*x as isize, y as isize) < 4
                        {
                            buf2.map[y][*x] = b'x';
                            true
                        } else {
                            false
                        }
                    })
                    .count() as u32
            })
            .sum::<u32>()
    };

    let mut total = 0;
    let mut taken = take_rolls(&mut map1, &mut map2);

    while taken != 0 {
        total += taken;
        map1 = map2.clone();
        taken = take_rolls(&mut map1, &mut map2);
    }

    total
}

#[derive(Clone)]
struct Map {
    map: Vec<Vec<u8>>,
}

impl Map {
    fn new(input: &str) -> Self {
        let map = input
            .trim()
            .lines()
            .map(|line| line.as_bytes().to_vec())
            .collect();

        Self { map }
    }

    fn count_neighbour(&self, x: isize, y: isize) -> u32 {
        let mut total = 0;

        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }

                let x = x + dx;
                let y = y + dy;

                if 0 <= y
                    && y < self.map.len() as isize
                    && 0 <= x
                    && x < self.map[y as usize].len() as isize
                {
                    let cell = self.map[y as usize][x as usize];
                    total += (cell == b'@') as u32
                }
            }
        }

        total
    }
}
