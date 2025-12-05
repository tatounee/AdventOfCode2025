pub fn part1(input: &str) -> u64 {
    let (db, items) = input.split_once("\n\n").unwrap();
    let db = Database::new(db);

    items
        .trim()
        .lines()
        .filter(|item| {
            let item = item.parse::<u64>().unwrap();
            db.contains(item)
        })
        .count() as u64
}

pub fn part2(input: &str) -> u64 {
    let (db, _) = input.split_once("\n\n").unwrap();
    let mut db = Database::new(db);

    loop {
        let prev_len = db.ranges.len();
        db.collapse();

        if db.ranges.len() == prev_len {
            break;
        }
    }

    db.assert_sorted();

    db.count_item()
}

#[derive(Debug, Default)]
struct Database {
    ranges: Vec<(u64, u64)>,
}

impl Database {
    fn new(input: &str) -> Self {
        let range = input
            .trim()
            .lines()
            .map(|range| {
                let (start, end) = range.split_once('-').unwrap();
                (start.parse::<u64>().unwrap(), end.parse::<u64>().unwrap())
            })
            .collect();

        Self { ranges: range }
    }

    fn contains(&self, item: u64) -> bool {
        self.ranges.iter().any(|range| contains(*range, item))
    }

    fn collapse(&mut self) {
        self.ranges.sort_unstable_by_key(|(start, _)| *start);

        let mut this = Self::default();
        for range in self.ranges.drain(..) {
            this.insert_range(range);
        }

        *self = this
    }

    /// Suppose que self.ranges est trié
    fn insert_range(&mut self, (start, end): (u64, u64)) {
        for range in &mut self.ranges {
            if contains(*range, start) {
                range.1 = range.1.max(end);
                return;
            }
            if contains(*range, end) {
                range.0 = range.0.min(start);
                return;
            }
        }

        self.ranges.push((start, end));
    }

    fn count_item(&self) -> u64 {
        self.ranges.iter().map(|(start, end)| end - start + 1).sum()
    }

    fn assert_sorted(&self) {
        for ranges in self.ranges.windows(2) {
            let (_, end0) = ranges[0];
            let (start1, _) = ranges[1];

            assert!(end0 < start1)
        }
    }
}

fn contains((start, end): (u64, u64), item: u64) -> bool {
    start <= item && item <= end
}
