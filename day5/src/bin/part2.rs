use std::ops::RangeInclusive;

fn main() {
    let input = aoc::parser::input::<String>();
    let (ranges, _ingredients) = input.trim().split_once("\n\n").unwrap();
    let ret = ranges
        .lines()
        .map(|line| line.split_once('-').unwrap())
        .map(|(l, r)| l.parse::<usize>().unwrap()..=r.parse().unwrap())
        .fold(Vec::new(), insert_ranges)
        .iter()
        .map(|range| range.end() - range.start() + 1)
        .sum::<usize>();
    println!("{ret}");
}

/// 1. we keep the ranges sorted by their first bound
/// 2. When two range overlap we extend the first one to the end of the second one
fn insert_ranges(
    mut ranges: Vec<RangeInclusive<usize>>,
    range: RangeInclusive<usize>,
) -> Vec<RangeInclusive<usize>> {
    match ranges.binary_search_by_key(range.start(), |r| *r.start()) {
        Ok(idx) => extend(&mut ranges[idx], range),
        Err(idx) => {
            if let Some(r) = ranges.get(idx)
                && (r.contains(range.start()) || r.contains(range.end()))
            {
                extend(&mut ranges[idx], range);
            } else {
                ranges.insert(idx, range);
            }
        }
    };

    // loop to merge all overlapping ranges
    let mut len = 0;
    while ranges.len() != len {
        len = ranges.len();
        let mut new_ranges = Vec::new();

        let mut iter = ranges.into_iter();
        // safe because we *must* have inserted at least one element above
        let mut current_range = iter.next().unwrap();
        for range in iter {
            if current_range.contains(range.start()) {
                extend(&mut current_range, range.clone());
            } else {
                new_ranges.push(current_range.clone());
                current_range = range;
            }
        }

        new_ranges.push(current_range);

        ranges = new_ranges;
    }
    ranges
}

fn extend(original: &mut RangeInclusive<usize>, new: RangeInclusive<usize>) {
    assert!(original.contains(new.start()) || original.contains(new.end()));
    *original = (*original.start()).min(*new.start())..=(*original.end()).max(*new.end());
}
