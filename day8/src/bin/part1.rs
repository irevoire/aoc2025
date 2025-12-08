use std::{
    collections::{HashMap, HashSet, VecDeque},
    str::FromStr,
};

use ordered_float::OrderedFloat;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
struct Coord([OrderedFloat<f32>; 3]);

impl Coord {
    pub fn distance(&self, other: &Self) -> OrderedFloat<f32> {
        self.0
            .iter()
            .zip(other.0.iter())
            .map(|(a, b)| (b - a).powi(2))
            .map(OrderedFloat::from)
            .sum()
    }
}

impl FromStr for Coord {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords = s
            .split(',')
            .map(|s| s.trim().parse().unwrap())
            .collect::<Vec<_>>();
        Ok(Self(coords.try_into().unwrap()))
    }
}

fn main() {
    let coords = aoc::parser::lines::<Coord>().collect::<Vec<_>>();
    let mut smallest_distance = coords
        .iter()
        .map(|coord| {
            let mut new_coords: Vec<(OrderedFloat<f32>, Coord)> = coords
                .iter()
                .filter(|c| *c != coord)
                .map(|c| (c.distance(coord), *c))
                .collect();
            new_coords.sort_unstable_by(|a, b| a.0.total_cmp(&b.0));
            new_coords.into()
        })
        .collect::<Vec<VecDeque<(OrderedFloat<f32>, Coord)>>>();

    let mut graph = coords
        .iter()
        .map(|c| (*c, Vec::new()))
        .collect::<HashMap<Coord, Vec<Coord>>>();

    for i in 0..1000 {
        // println!("Added {i} link in total");
        let (a, b) = pop(&coords, &mut smallest_distance);
        graph.get_mut(&a).unwrap().push(b);
        graph.get_mut(&b).unwrap().push(a);
        // let circuit = count_circuit(&graph);
        // println!("Circuit {circuit:?}");
    }
    let circuit = count_circuit(&graph);
    let ret = circuit.iter().take(3).product::<usize>();
    println!("{ret}");
}

fn count_circuit(graph: &HashMap<Coord, Vec<Coord>>) -> Vec<usize> {
    let mut circuit = Vec::new();
    let mut explored = HashSet::<Coord>::new();
    for (root, links) in graph.iter() {
        if !explored.insert(*root) {
            continue;
        }
        let mut ret = 1;
        let mut explore = links.clone();

        while let Some(next) = explore.pop() {
            if !explored.insert(next) {
                continue;
            }
            ret += 1;
            explore.extend(&graph[&next]);
        }
        circuit.push(ret);
    }
    circuit.sort_unstable();
    circuit.reverse();

    circuit
}

fn pop(coords: &[Coord], distances: &mut [VecDeque<(OrderedFloat<f32>, Coord)>]) -> (Coord, Coord) {
    let (idx, _min) = distances
        .iter()
        .enumerate()
        .filter(|(_idx, vec)| !vec.is_empty())
        .min_by(|(_idx, a), (_, b)| a[0].0.total_cmp(&b[0].0))
        .unwrap();

    let (_dist, right) = distances[idx].pop_front().unwrap();
    let left = coords[idx];

    // we must also remove it from the other matching coord
    let idx = coords.iter().position(|c| *c == right).unwrap();
    let (_dist, truc) = distances[idx].pop_front().unwrap();
    assert_eq!(left, truc);

    // println!("Connected {left:?} and {right:?} which have a distance of {_dist}");

    (left, right)
}
