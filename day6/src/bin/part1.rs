use aoc::Grid;

fn main() {
    let input = aoc::parser::input::<String>();
    let grid = input
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let grid = Grid::from(grid);
    let ret = grid
        .columns()
        .map(|col| {
            let identity = match **col.last().unwrap() {
                "+" => 0,
                "*" => 1,
                _ => panic!(),
            };
            col[0..col.len() - 1]
                .iter()
                .map(|s| s.parse::<usize>().unwrap())
                .fold(identity, |acc, v| match **col.last().unwrap() {
                    "+" => acc + v,
                    "*" => acc * v,
                    _ => panic!(),
                })
        })
        .sum::<usize>();
    println!("{grid}");
    println!("{ret}");
}
