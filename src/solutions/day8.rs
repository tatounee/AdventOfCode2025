use std::{
    collections::{BinaryHeap, HashMap},
    fmt,
};

pub fn part1(input: &str) -> u64 {
    let boxes: Vec<Vec3> = input
        .trim()
        .lines()
        .map(|jbox| {
            let mut coords = jbox.split(',');
            Vec3(
                coords.next().unwrap().parse::<u64>().unwrap(),
                coords.next().unwrap().parse::<u64>().unwrap(),
                coords.next().unwrap().parse::<u64>().unwrap(),
            )
        })
        .collect();

    let mut joins: BinaryHeap<Join> = boxes
        .iter()
        .enumerate()
        .map(|(i, box0)| {
            boxes
                .iter()
                .enumerate()
                .filter(move |(j, _)| i < *j)
                .map(move |(j, box1)| Join::new((*box0, i), (*box1, j)))
        })
        .flatten()
        .collect();

    let mut circuits = DisjoinSet::new(boxes.len());
    for _ in 0..1000 {
        let join = joins.pop().unwrap();
        circuits.union(join.box0_idx, join.box1_idx);
    }

    let mut circuits_root: HashMap<usize, u64> = HashMap::new();
    for jbox in 0..boxes.len() {
        let circuit = circuits.find(jbox);
        *circuits_root.entry(circuit.root).or_default() += 1u64;
    }

    let mut circuits_len: Vec<u64> = circuits_root.values().copied().collect();
    circuits_len.sort_unstable();
    let len = circuits_len.len();
    circuits_len[len - 1] * circuits_len[len - 2] * circuits_len[len - 3]
}

pub fn part2(input: &str) -> u64 {
    let boxes: Vec<Vec3> = input
        .trim()
        .lines()
        .map(|jbox| {
            let mut coords = jbox.split(',');
            Vec3(
                coords.next().unwrap().parse::<u64>().unwrap(),
                coords.next().unwrap().parse::<u64>().unwrap(),
                coords.next().unwrap().parse::<u64>().unwrap(),
            )
        })
        .collect();

    let mut joins: BinaryHeap<Join> = boxes
        .iter()
        .enumerate()
        .map(|(i, box0)| {
            boxes
                .iter()
                .enumerate()
                .filter(move |(j, _)| i < *j)
                .map(move |(j, box1)| Join::new((*box0, i), (*box1, j)))
        })
        .flatten()
        .collect();

    let mut circuits = DisjoinSet::new(boxes.len());
    let mut last_join = *joins.peek().unwrap();

    for _ in 0..joins.len() {
        let join = joins.pop().unwrap();
        let connected = circuits.union(join.box0_idx, join.box1_idx);

        if connected {
            last_join = join
        }
    }

    let box0 = boxes[last_join.box0_idx];
    let box1 = boxes[last_join.box1_idx];
    box0.0 * box1.0
}

#[derive(Debug, Clone, Copy)]
struct Join {
    box0_idx: usize,
    box1_idx: usize,
    dist: u64,
}

impl Join {
    fn new((box0, i): (Vec3, usize), (box1, j): (Vec3, usize)) -> Self {
        let dist = box0.squared_dist(&box1);
        Self {
            box0_idx: i,
            box1_idx: j,
            dist,
        }
    }
}

#[derive(Clone, Copy)]
struct Vec3(u64, u64, u64);

impl Vec3 {
    fn squared_dist(&self, other: &Self) -> u64 {
        let xx = self.0.abs_diff(other.0).pow(2);
        let yy = self.1.abs_diff(other.1).pow(2);
        let zz = self.2.abs_diff(other.2).pow(2);
        xx + yy + zz
    }
}

impl PartialEq for Join {
    fn eq(&self, other: &Self) -> bool {
        self.dist == other.dist
    }
}

impl Eq for Join {}

impl PartialOrd for Join {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.dist.partial_cmp(&self.dist)
    }
}

impl Ord for Join {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.dist.partial_cmp(&other.dist).unwrap()
    }
}

struct DisjoinSet {
    data: Vec<DisjoinSetItem>,
}

#[derive(Clone)]
struct DisjoinSetItem {
    value: usize,
    root: usize,
    rank: usize,
}

impl DisjoinSetItem {
    fn new(value: usize, root: usize, rank: usize) -> Self {
        Self { value, root, rank }
    }
}

impl DisjoinSet {
    fn new(len: usize) -> Self {
        let data = (0..len).map(|x| DisjoinSetItem::new(x, x, 0)).collect();

        Self { data }
    }

    /// Return `true` if two subset have been unified.
    fn union(&mut self, i: usize, j: usize) -> bool {
        let root_i = self.find(i);
        let root_j = self.find(j);

        if root_i.value == root_j.value {
            return false;
        }

        let (root_i, root_j) = if root_i.rank < root_j.rank {
            (&mut self.data[root_i.value], root_j)
        } else {
            (&mut self.data[root_j.value], root_i)
        };

        assert!(root_i.rank <= root_j.rank);

        root_i.root = root_j.value;

        if root_i.rank == root_j.rank {
            self.data[root_j.value].rank += 1;
        }

        true
    }

    fn find(&mut self, i: usize) -> DisjoinSetItem {
        if self.data[i].root == i {
            self.data[i].clone()
        } else {
            let root = self.find(self.data[i].root);
            self.data[i].root = root.value;
            root
        }
    }
}

impl fmt::Debug for DisjoinSetItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(
            "{:02} -> {:02} | {}",
            self.value, self.root, self.rank
        ))
    }
}
