fn main() {
    let grid = aoc::parser::lines()
        .map(|line: String| line.chars().collect())
        .collect();
    let mut prev = aoc::Grid::new();
    let mut grid = aoc::Grid::from(grid);
    println!("{grid}");

    let mut pq_removed = 0;

    loop {
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

        pq_removed += ret.len();

        // redraw the map for fun
        grid = grid.map_with_coord(|c, v| if ret.contains(&c) { 'x' } else { v });
        println!(
            "{}",
            grid.to_string()
                .chars()
                .filter(|c| *c != ' ')
                .collect::<String>()
        );
        std::thread::sleep(std::time::Duration::from_millis(200));
        grid = grid.map_with_coord(|c, v| if ret.contains(&c) { '.' } else { v });
        println!(
            "{}",
            grid.to_string()
                .chars()
                .filter(|c| *c != ' ')
                .collect::<String>()
        );
        std::thread::sleep(std::time::Duration::from_millis(200));
        if grid == prev {
            break;
        } else {
            prev = grid.clone();
        }
    }
    println!("{}", pq_removed);
}
