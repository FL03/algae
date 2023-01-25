#[cfg(feature = "graph")]
#[cfg(test)]
mod directed_graph_tests {
    use algae::graph::{directed::DirectedGraph, Graphable};

    #[test]
    fn test_add_node() {
        let tmp = ["a", "b", "c"]
            .iter()
            .cloned()
            .map(String::from)
            .collect::<Vec<_>>();

        let mut graph = DirectedGraph::default();
        graph.add_node("a");
        graph.add_node("b");
        graph.add_node("c");
        assert_eq!(
            graph.nodes(),
            [&tmp[0], &tmp[1], &tmp[2]].iter().cloned().collect()
        );
    }

    #[test]
    fn test_add_edge() {
        let mut graph = DirectedGraph::default();

        graph.add_edge(("a", "b", 5));
        graph.add_edge(("c", "a", 7));
        graph.add_edge(("b", "c", 10));

        let expected_edges = [
            (&String::from("a"), &String::from("b"), 5),
            (&String::from("c"), &String::from("a"), 7),
            (&String::from("b"), &String::from("c"), 10),
        ];
        for edge in expected_edges.iter() {
            assert!(graph.edges().contains(edge));
        }
    }

    #[test]
    fn test_neighbours() {
        let mut graph = DirectedGraph::default();

        graph.add_edge(("a", "b", 5));
        graph.add_edge(("b", "c", 10));
        graph.add_edge(("c", "a", 7));

        assert_eq!(
            graph.neighbours("a").unwrap(),
            &vec![(String::from("b"), 5)]
        );
    }

    #[test]
    fn test_contains() {
        let mut graph = DirectedGraph::default();
        graph.add_node("a");
        graph.add_node("b");
        graph.add_node("c");
        assert!(graph.contains("a"));
        assert!(graph.contains("b"));
        assert!(graph.contains("c"));
        assert!(!graph.contains("d"));
    }
}

#[cfg(feature = "graph")]
#[cfg(test)]
mod undirected_graph_tests {
    use algae::graph::{undirected::UndirectedGraph, Graphable};

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
}
