use aoc::Coord;

fn main() {
    let coords = aoc::parser::lines::<Coord>().collect::<Vec<_>>();

    let max = coords
        .iter()
        .flat_map(|left| coords.iter().map(move |right| (left, right)))
        .map(|(left, right)| (left.x.abs_diff(right.x) + 1) * (left.y.abs_diff(right.y) + 1))
        .max()
        .unwrap();

    println!("{max}");
}
