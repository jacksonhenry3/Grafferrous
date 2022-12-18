use criterion::{black_box, criterion_group, criterion_main, Criterion};
use graph::{generate_cycle_graph, generate_grid_graph, Graph, ID};

fn benchmark_empty_creation(c: &mut Criterion) {
    c.bench_function("empty", |b| b.iter(|| Graph::<u32>::new()));
}

fn benchmark_add_1000_nodes(c: &mut Criterion) {
    c.bench_function("add 1000 nodes", |b| {
        b.iter(|| {
            let mut g = Graph::<u32>::new();
            for i in 0..1000 {
                g.add_node(ID(i));
            }
        })
    });
}

fn benchmark_add_100000_nodes(c: &mut Criterion) {
    c.bench_function("add 100000 nodes", |b| {
        b.iter(|| {
            let mut g = Graph::<u32>::new();
            for i in 0..100000 {
                g.add_node(ID(i));
            }
        })
    });
}

fn benchmark_add_1000_edges(c: &mut Criterion) {
    c.bench_function("add 1000 edges", |b| {
        b.iter(|| {
            let mut g = Graph::<u32>::new();
            for i in 0..1000 {
                g.add_node(ID(i));
            }
            for i in 0..1000 {
                g.add_edge(ID(i), ID((i + 1) % 1000));
            }
        })
    });
}

fn benchmark_cycle_creation(c: &mut Criterion) {
    c.bench_function("cycle 10_000", |b| {
        b.iter(|| generate_cycle_graph::<u32>(black_box(10_000)))
    });
}

fn benchmark_grid(c: &mut Criterion) {
    c.bench_function("grid 100x100", |b| {
        b.iter(|| generate_grid_graph::<u32>(black_box(100), black_box(100)))
    });
}

criterion_group!(
    benches,
    benchmark_empty_creation,
    benchmark_add_1000_nodes,
    benchmark_add_100000_nodes,
    benchmark_add_1000_edges,
    benchmark_cycle_creation,
    benchmark_grid
);
criterion_main!(benches);
