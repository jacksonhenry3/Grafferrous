// #![allow(dead_code)]

use grafferous::{count_paths, generate_cycle_graph, generate_grid_graph, Graph};

#[test]
fn test_generate_cycle_graph() {
    let g = generate_cycle_graph::<u32>(10_000);
    assert_eq!(g.nodes.len(), 10_000);
    assert_eq!(g.edges.len(), 10_000);
}

#[test]
fn test_generate_grid_graph() {
    let g = generate_grid_graph::<u32>(100, 100);
    assert_eq!(g.nodes.len(), 10_000);
    assert_eq!(g.edges.values().flatten().count(), 2 * (20_000 - 100 - 100));
}

#[test]
fn test_empty_creation() {
    let g = Graph::<usize, u32>::new();
    assert_eq!(g.nodes.len(), 0);
    assert_eq!(g.edges.len(), 0);
}

#[test]
fn test_count_paths() {
    let mut g = Graph::<usize, u32>::new();

    g.add_directed_edge(0, 1);
    g.add_directed_edge(1, 2);
    g.add_directed_edge(2, 3);
    g.add_directed_edge(3, 4);
    g.add_directed_edge(4, 5);
    g.add_directed_edge(5, 6);

    assert_eq!(count_paths(&g, &0, &6, None), 1);
    assert_eq!(count_paths(&g, &1, &6, None), 1);
    assert_eq!(count_paths(&g, &2, &6, None), 1);
    assert_eq!(count_paths(&g, &3, &6, None), 1);
    assert_eq!(count_paths(&g, &4, &6, None), 1);
    assert_eq!(count_paths(&g, &5, &6, None), 1);
    assert_eq!(count_paths(&g, &6, &6, None), 1);
}

//test count paths fails on cycles
#[test]
#[should_panic]
fn test_count_paths_cycle() {
    let mut g = Graph::<usize, u32>::new();

    g.add_edge(0, 1);
    g.add_edge(1, 2);
    g.add_edge(2, 3);
    g.add_edge(3, 4);
    g.add_edge(4, 5);
    g.add_edge(5, 6);
    g.add_edge(6, 0);

    count_paths(&g, &0, &6, None);
}

//test teh graph macro
#[test]
fn test_graph_macro() {
    let g: Graph<i32, ()> = grafferous::graph! {
        0 => 1,
        1 => 2,
        2 => 3,
        3 => 4,
        4 => 5,
        5 => 6
    };

    assert_eq!(g.nodes.len(), 7);
    assert_eq!(g.edge_tuples().len(), 6);

    let g: Graph<i32, ()> = grafferous::graph! {
        0 ; 1,
        1 ; 2,
        2 ; 3,
        3 ; 4,
        4 ; 5,
        5 ; 6
    };

    assert_eq!(g.nodes.len(), 7);
    assert_eq!(g.edge_tuples().len(), 12);
}

//random graph test
#[test]
fn test_random_graph() {
    let g = grafferous::generate_random_graph::<u32>(100, 0.1);
    assert_eq!(g.nodes.len(), 100);
}
