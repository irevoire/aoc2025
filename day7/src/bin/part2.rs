use std::collections::HashMap;

use aoc::{Coord, Direction, Grid};

fn main() {
    let grid = aoc::parser::lines::<String>()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();
    let grid = Grid::from(grid);

    let start = grid.position(|&c| c == 'S').unwrap();
    let ret = explore(&grid, start, &mut HashMap::new());

    println!("{ret}");
}

fn explore(grid: &Grid<char>, coord: Coord, cache: &mut HashMap<Coord, usize>) -> usize {
    if let Some(ret) = cache.get(&coord) {
        return *ret;
    }

    match grid[coord] {
        '.' | 'S' => {
            let coord = coord + Direction::Down;
            let ret = if grid.get(coord).is_some() {
                explore(grid, coord, cache)
            } else {
                1
            };
            cache.insert(coord, ret);
            ret
        }
        '^' => {
            let left = coord + Direction::Left;
            let left = explore(grid, left, cache);
            let right = coord + Direction::Right;
            let right = explore(grid, right, cache);
            cache.insert(coord, left + right);
            left + right
        }
        _ => panic!(),
    }
}
