// #![allow(dead_code)]

use graph::{generate_cycle_graph, generate_grid_graph, Graph};

#[test]
fn test_generate_cycle_graph(){
    let g = generate_cycle_graph::<u32>(10_000);
    assert_eq!(g.nodes.len(), 10_000);
    assert_eq!(g.edges.len(), 10_000);
}

#[test]
fn test_generate_grid_graph(){
    let g = generate_grid_graph::<u32>(100, 100);
    assert_eq!(g.nodes.len(), 10_000);
    assert_eq!(g.edges.values().flatten().count(), 2*(20_000-100-100));
}

#[test]
fn test_empty_creation(){
    let g = Graph::<usize,u32>::new();
    assert_eq!(g.nodes.len(), 0);
    assert_eq!(g.edges.len(), 0);
}