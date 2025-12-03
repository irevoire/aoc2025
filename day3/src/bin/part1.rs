use aoc::Itertools;

fn main() {
    let ret = aoc::parser::lines::<String>()
        .map(|bat| {
            let max = bat.len()
                - 2
                - bat.as_bytes()[0..bat.len() - 1]
                    .iter()
                    .rev()
                    .position_max()
                    .unwrap();
            let second_max = bat.as_bytes()[max + 1..].iter().max().unwrap();
            (bat.as_bytes()[max] - b'0') as usize * 10 + (*second_max - b'0') as usize
        })
        .sum::<usize>();
    println!("{ret}");
}
