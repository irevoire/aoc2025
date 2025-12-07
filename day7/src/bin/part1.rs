use std::collections::{HashSet, VecDeque};

use aoc::{Direction, Grid};

fn main() {
    let grid = aoc::parser::lines::<String>()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();
    let mut grid = Grid::from(grid);

    let start = grid.position(|&c| c == 'S').unwrap();
    let explore = vec![start];
    let mut explore = VecDeque::from(explore);
    let mut split = HashSet::new();

    while let Some(coord) = explore.pop_front() {
        match grid[coord] {
            '.' | 'S' => {
                grid[coord] = '|';
                let coord = coord + Direction::Down;
                if grid.get(coord).is_some() {
                    explore.push_back(coord);
                }
            }
            '|' => continue,
            '^' => {
                if !split.insert(coord) {
                    continue;
                }
                let left = coord + Direction::Left;
                if grid.get(left).is_some() {
                    explore.push_back(left);
                }
                let right = coord + Direction::Right;
                if grid.get(right).is_some() {
                    explore.push_back(right);
                }
                continue;
            }
            _ => panic!(),
        }
        println!("{grid}");
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    println!("{}", split.len());
}
