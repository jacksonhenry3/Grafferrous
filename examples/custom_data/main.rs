use std::vec;

use graph::Graph;

#[derive(Debug,Default)]
struct CustomData {
    a: usize,
    b: usize,
    c: Vec<u32>,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct CustomID {
    x: usize,
    y: usize,
}

fn main() {

    let mut g = Graph::<CustomID,CustomData>::new();

    g.add_node(CustomID{x: 0, y: 0});
    g.add_node(CustomID{x: 0, y: 1});
    g.add_node_with_data(CustomID{x: 1, y: 0}, CustomData{a: 1, b: 2, c: vec![1,2,3]}); // add node with data

    //Modify node data
    if let Some(node_data) = g.node_data.get_mut(&CustomID{x: 0, y: 0}) {
        *node_data = CustomData{a: 1, b: 2, c: vec![1,2,3]};
    }

    if let Some(node_data) = g.node_data.get_mut(&CustomID{x: 0, y: 1}) {
        node_data.a = 1;
        node_data.b = 2;
        node_data.c = vec![1,2,3];
    }



    println!("{:?}", g);

}