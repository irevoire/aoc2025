fn main() {
    let input = aoc::parser::input::<String>();
    let (ranges, ingredients) = input.trim().split_once("\n\n").unwrap();
    let ranges = ranges
        .lines()
        .map(|line| line.split_once('-').unwrap())
        .map(|(l, r)| l.parse::<usize>().unwrap()..=r.parse().unwrap())
        .collect::<Vec<_>>();
    let ret = ingredients
        .lines()
        .map(|ingredient| ingredient.parse().unwrap())
        .filter(|ingredient| ranges.iter().any(|range| range.contains(ingredient)))
        .count();
    println!("{ret}");
}
