// bench.rs
#![feature(test)]

extern crate test;
use algae::graph::{DirectedGraph, Edge, Graph,};
use test::Bencher;

const TEST_EDGES: [(&str, &str, usize); 5] = [
    ("a", "b", 5),
    ("c", "a", 7),
    ("b", "c", 10),
    ("d", "c", 10),
    ("e", "f", 10),
];

#[bench]
fn bench_directed(b: &mut Bencher) {
    let mut graph = DirectedGraph::<&str, usize>::new();
    b.iter(|| {
        TEST_EDGES
            .into_iter()
            .map(|i| Edge::from(i))
            .for_each(|i| graph.add_edge(i));
    });
}
