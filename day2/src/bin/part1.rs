fn main() {
    let ret = aoc::parser::input::<String>()
        .trim()
        .split(",")
        .flat_map(|s| {
            let (l, r) = s.split_once("-").unwrap();
            l.parse::<usize>().unwrap()..=r.parse::<usize>().unwrap()
        })
        .map(|n| n.to_string())
        .filter(|s| s.len() % 2 == 0)
        .filter(|s| s[0..s.len() / 2] == s[s.len() / 2..])
        .map(|s| s.parse::<usize>().unwrap())
        .sum::<usize>();
    println!("{ret}");
}
