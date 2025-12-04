fn main() {
    let grid = aoc::parser::lines()
        .map(|line: String| line.chars().collect())
        .collect();
    let grid = aoc::Grid::from(grid);
    let ret = grid
        .enumerate()
        .filter(|(_, val)| **val == '@')
        .map(|(coord, _)| {
            (
                coord,
                coord
                    .chebyshev_adjacent()
                    .filter_map(|coord| grid.get(coord))
                    .filter(|val| **val == '@')
                    .count(),
            )
        })
        .filter(|(_, count)| *count < 4)
        .map(|(c, _)| c)
        .collect::<Vec<_>>();

    // redraw the map for fun
    let grid = grid.map_with_coord(|c, v| if ret.contains(&c) { 'x' } else { v });
    println!("{grid}");
    println!("{}", ret.len());
}
