#[cfg(test)]
use algae_graph::{undirected::UndirectedGraph, Graphable};

#[test]
fn test_add_edge() {
    let mut graph = UndirectedGraph::default();

    graph.add_edge(("a", "b", 5));
    graph.add_edge(("b", "c", 10));
    graph.add_edge(("c", "a", 7));

    let expected_edges = [
        (&String::from("a"), &String::from("b"), 5),
        (&String::from("b"), &String::from("a"), 5),
        (&String::from("c"), &String::from("a"), 7),
        (&String::from("a"), &String::from("c"), 7),
        (&String::from("b"), &String::from("c"), 10),
        (&String::from("c"), &String::from("b"), 10),
    ];
    for edge in expected_edges.iter() {
        assert!(graph.edges().contains(edge));
    }
}

#[test]
fn test_neighbours() {
    let mut graph = UndirectedGraph::default();

    graph.add_edge(("a", "b", 5));
    graph.add_edge(("b", "c", 10));
    graph.add_edge(("c", "a", 7));

    assert_eq!(
        graph.neighbours("a").unwrap(),
        &vec![(String::from("b"), 5), (String::from("c"), 7)]
    );
}
