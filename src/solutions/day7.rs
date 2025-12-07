use crate::utils::dbg_map;

pub fn part1(input: &str) -> u32 {
    let mut grid = Grid::new(input);

    grid.simulate();

    grid.count_split()
}
pub fn part2(input: &str) -> u64 {
    let grid = Grid::new(input);

    let mut timelines = vec![vec![0; grid.grid[0].len()]; grid.grid.len()];
    grid.timeline(grid.generator_x(), 0, &mut timelines)
}

struct Grid {
    grid: Vec<Vec<Cell>>,
}

impl Grid {
    fn new(input: &str) -> Self {
        let grid = input
            .trim()
            .lines()
            .map(|line| {
                line.as_bytes()
                    .into_iter()
                    .copied()
                    .map(Cell::from)
                    .collect()
            })
            .collect();

        Self { grid }
    }

    fn neighbours(&self, x: usize, y: usize) -> [Cell; 9] {
        let mut neighbours = [Cell::Empty; 9];

        for dy in -1..=1 {
            for dx in -1..=1 {
                let y = y as isize + dy;
                let x = x as isize + dx;

                if 0 <= y
                    && 0 <= x
                    && let Some(cell) = self
                        .grid
                        .get(y as usize)
                        .and_then(|line| line.get(x as usize))
                {
                    neighbours[((dy + 1) * 3 + dx + 1) as usize] = *cell
                }
            }
        }

        neighbours
    }

    fn simulate(&mut self) {
        for y in 1..self.grid.len() {
            self.simulate_line(y)
        }
    }

    fn simulate_line(&mut self, y: usize) {
        let line = self.grid[y]
            .iter()
            .enumerate()
            .map(|(x, cell)| cell.next(self.neighbours(x, y)))
            .collect();

        self.grid[y] = line;
    }

    fn count_split(&self) -> u32 {
        self.grid
            .iter()
            .enumerate()
            .skip(1)
            .map(|(y, line)| {
                line.iter().enumerate().filter(move |(x, cell)| {
                    **cell == Cell::Spliter && self.grid[y - 1][*x] == Cell::Beam
                })
            })
            .flatten()
            .count() as u32
    }

    fn generator_x(&self) -> usize {
        self.grid[0]
            .iter()
            .position(|cell| *cell == Cell::Generator)
            .unwrap()
    }

    fn timeline(&self, x: usize, y: usize, timelines: &mut [Vec<u64>]) -> u64 {
        if timelines[y][x] != 0 {
            timelines[y][x]
        } else if y + 1 == self.grid.len() {
            timelines[y][x] = 1;
            1
        } else {
            timelines[y][x] = match self.grid[y][x] {
                Cell::Generator | Cell::Beam | Cell::Empty => self.timeline(x, y + 1, timelines),
                Cell::Spliter => {
                    self.timeline(x + 1, y, timelines) + self.timeline(x - 1, y, timelines)
                }
            };
            timelines[y][x]
        }
    }

    fn dbg(&self) {
        let grid: Vec<_> = self
            .grid
            .iter()
            .map(|line| line.iter().map(Cell::as_byte).collect())
            .collect();
        dbg_map(&grid);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Generator,
    Beam,
    Spliter,
    Empty,
}

impl Cell {
    fn next(self, neighbours: [Cell; 9]) -> Self {
        match self {
            Cell::Empty => {
                if matches!(neighbours[1], Cell::Generator | Cell::Beam) {
                    Cell::Beam
                } else if neighbours[2] == Cell::Beam && neighbours[5] == Cell::Spliter {
                    Cell::Beam
                } else if neighbours[0] == Cell::Beam && neighbours[3] == Cell::Spliter {
                    Cell::Beam
                } else {
                    Cell::Empty
                }
            }
            _ => self,
        }
    }

    fn as_byte(&self) -> u8 {
        match self {
            Cell::Generator => b'S',
            Cell::Beam => b'|',
            Cell::Spliter => b'^',
            Cell::Empty => b'.',
        }
    }
}

impl From<u8> for Cell {
    fn from(value: u8) -> Self {
        match value {
            b'S' => Cell::Generator,
            b'|' => Cell::Beam,
            b'^' => Cell::Spliter,
            b'.' => Cell::Empty,
            _ => panic!("Unknow char: {}", value as char),
        }
    }
}
