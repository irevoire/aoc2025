fn main() {
    let ret = aoc::parser::input::<String>()
        .trim()
        .split(",")
        .flat_map(|s| {
            let (l, r) = s.split_once("-").unwrap();
            l.parse::<usize>().unwrap()..=r.parse::<usize>().unwrap()
        })
        .map(|n| n.to_string())
        .filter(|s| {
            (1..=s.len() / 2)
                .filter(|n| s.len() % n == 0)
                .any(|n| (0..s.len() / n).all(|idx| s[n * idx..n * (idx + 1)] == s[0..n]))
        })
        .map(|s| s.parse::<usize>().unwrap())
        .sum::<usize>();
    println!("{ret}");
}
