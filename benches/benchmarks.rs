#![allow(unused)]
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use graph::{generate_cycle_graph, generate_grid_graph, Graph};

fn benchmark_empty_creation(c: &mut Criterion) {
    c.bench_function("empty", |b| b.iter(Graph::<usize, u32>::new));
}
fn benchmark_addition(c: &mut Criterion) {
    c.bench_function("addition", |b| {
        b.iter(|| black_box(345.1) + black_box(799.2234))
    });
}

fn benchmark_add_1_nodes(c: &mut Criterion) {
    c.bench_function("add 1 nodes", |b| {
        b.iter(|| {
            let mut g = Graph::<usize, u32>::new();
            g.add_node(0);
        })
    });
}

fn benchmark_add_100000_nodes(c: &mut Criterion) {
    c.bench_function("add 100000 nodes", |b| {
        b.iter(|| {
            let mut g = Graph::<usize, u32>::new();
            for i in 0..100000 {
                g.add_node(i);
            }
        })
    });
}

fn benchmark_add_1000_edges(c: &mut Criterion) {
    c.bench_function("add 1000 edges", |b| {
        b.iter(|| {
            let mut g = Graph::<usize, u32>::new();
            for i in 0..1000 {
                g.add_node(i);
            }
            for i in 0..1000 {
                g.add_edge(i, (i + 1) % 1000);
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
    benchmark_addition,
    benchmark_empty_creation,
    benchmark_add_1_nodes,
    benchmark_add_100000_nodes,
    benchmark_add_1000_edges,
    benchmark_cycle_creation,
    benchmark_grid
);

criterion_main!(benches);
