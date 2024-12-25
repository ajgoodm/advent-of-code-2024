use grid::Grid;
use utils::AocBufReader;

fn main() {
    part_1(AocBufReader::from_string("aoc/src/day_25/data/part_1.txt"));
}

fn part_1(input: AocBufReader) {
    let (locks, keys) = parse_input(input);
    println!("part 1: {}", part_1_inner(locks, keys),);
}

fn part_1_inner(locks: Vec<Lock>, keys: Vec<Key>) -> usize {
    let mut count: usize = 0;
    for lock in locks {
        for key in keys.iter() {
            if lock.key_fits(key) {
                count += 1;
            }
        }
    }

    count
}

fn parse_input(input: impl Iterator<Item = String>) -> (Vec<Lock>, Vec<Key>) {
    let mut locks: Vec<Lock> = Vec::new();
    let mut keys: Vec<Key> = Vec::new();

    let mut object: Vec<String> = Vec::new();
    for line in input {
        if line.is_empty() {
            let lock_or_key = Grid::from_line_iter(object.into_iter());
            if lock_or_key.row(0).iter().all(|&c| c == '#') {
                locks.push(Lock::new(lock_or_key))
            } else if lock_or_key
                .row(lock_or_key.n_rows - 1)
                .iter()
                .all(|&c| c == '#')
            {
                keys.push(Key::new(lock_or_key));
            } else {
                panic!("Not a lock or a key; what do? {:?}", lock_or_key);
            }

            object = Vec::new(); // start on the next one
        } else {
            object.push(line);
        }
    }

    (locks, keys)
}

struct Lock {
    grid: Grid<char>,
}

impl Lock {
    fn new(grid: Grid<char>) -> Self {
        Self { grid }
    }

    fn key_fits(&self, key: &Key) -> bool {
        if self.grid.n_rows != key.grid.n_rows {
            panic!("I thought all keys and locks had the same number of rows")
        }
        let n_rows = self.grid.n_rows;

        self.grid
            .cols()
            .zip(key.grid.cols())
            .all(|(lock_pins, key_profile)| {
                lock_pins.iter().filter(|&c| *c == '#').count()
                    + key_profile.iter().filter(|&c| *c == '#').count()
                    <= n_rows
            })
    }
}

struct Key {
    grid: Grid<char>,
}

impl Key {
    fn new(grid: Grid<char>) -> Self {
        Self { grid }
    }
}
