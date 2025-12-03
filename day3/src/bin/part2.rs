use aoc::Itertools;

fn main() {
    let ret = aoc::parser::lines::<String>()
        .map(|bat| {
            (0..12)
                .rev()
                .scan(0, |start, rem| {
                    let pos = bat.len()
                        - 1
                        - rem
                        - bat.as_bytes()[*start..bat.len() - rem]
                            .iter()
                            .rev()
                            .position_max()
                            .unwrap();
                    *start = pos + 1;

                    Some((bat.as_bytes()[pos] - b'0') as usize)
                })
                .fold(0, |acc, digit| acc * 10 + digit)
        })
        .sum::<usize>();
    println!("{ret}");
}
