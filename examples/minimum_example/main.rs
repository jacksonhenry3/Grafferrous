use graph::Graph;

fn main() {
    let mut g = Graph::<usize, u32>::new();

    g.add_node(0);
    g.add_node(1);
    g.add_node(2);
    g.add_node(3);

    g.add_edge(0, 1);
    g.add_edge(1, 2);
    g.add_edge(2, 3);

    println!("{:?}", g);
}
