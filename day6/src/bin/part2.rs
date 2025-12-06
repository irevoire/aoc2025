use aoc::Grid;

fn main() {
    let input = aoc::parser::input::<String>();
    let lines: Vec<_> = input.lines().collect();

    let math = lines[0..lines.len() - 1]
        .iter()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let math = Grid::from(math);
    let mut math_iter = math.columns();

    let op = lines.last().unwrap();
    let mut op_iter = op.char_indices();

    let mut ret = 0;
    let (mut last_idx, mut last_op) = op_iter.find(|(_, op)| *op != ' ').unwrap();

    while let Some((idx, op)) = op_iter.find(|(_, op)| *op != ' ') {
        let identity = match last_op {
            '+' => 0,
            '*' => 1,
            _ => panic!(),
        };
        let take = idx - last_idx - 1;
        let col_ret = math_iter
            .by_ref()
            .take(take)
            .map(|v| {
                v.iter().fold(0, |acc, n| {
                    if **n == ' ' {
                        acc
                    } else {
                        acc * 10 + n.to_string().parse::<usize>().unwrap()
                    }
                })
            })
            .fold(identity, |acc, v| match last_op {
                '+' => acc + v,
                '*' => acc * v,
                _ => panic!(),
            });
        // throw the empty column
        math_iter.next();
        ret += col_ret;
        last_idx = idx;
        last_op = op;
    }

    let identity = match last_op {
        '+' => 0,
        '*' => 1,
        _ => panic!(),
    };
    let take = lines.len() - last_idx - 1;
    let col_ret = math_iter
        .by_ref()
        .take(take)
        .map(|v| {
            v.iter().fold(0, |acc, n| {
                if **n == ' ' {
                    acc
                } else {
                    acc * 10 + n.to_string().parse::<usize>().unwrap()
                }
            })
        })
        .fold(identity, |acc, v| match last_op {
            '+' => acc + v,
            '*' => acc * v,
            _ => panic!(),
        });
    ret += col_ret;

    println!("{ret}");
}
