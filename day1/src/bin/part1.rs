fn main() {
    let n = aoc::parser::lines::<String>()
        .scan(50isize, |point, dir| {
            let n = match dir.split_at(1) {
                ("L", n) => -n.parse::<isize>().unwrap(),
                ("R", n) => n.parse::<isize>().unwrap(),
                _ => unreachable!(),
            };
            *point = (*point + n + 100) % 100;
            Some(*point)
        })
        .filter(|n| *n == 0)
        .count();
    println!("{n}");
}
