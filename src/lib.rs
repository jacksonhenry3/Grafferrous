#![allow(unused)]

use std::fmt::Debug;
use core::hash::Hash;
use fnv::{FnvHashMap, FnvHashSet};


#[derive(Debug, PartialEq, Eq, Clone)]
/// A graph data structure with nodes of type `NodeDataType` and edges between them.
pub struct Graph<IDDataType,NodeDataType> where IDDataType: Debug + PartialEq + Eq + Hash + Clone + Copy{
    /// A map from node IDs to their associated data.
    pub node_data: FnvHashMap<IDDataType, NodeDataType>,
    /// A map from node IDs to a vector of their outgoing edges.
    pub edges: FnvHashMap<IDDataType, Vec<IDDataType>>,
    /// A map from node IDs to a vector of their incoming edges.
    pub reverse_edges: FnvHashMap<IDDataType, Vec<IDDataType>>,
    /// A vector of all node IDs in the graph.
    pub nodes: Vec<IDDataType>,
}

impl<IDDataType,NodeDataType: Default> Graph<IDDataType,NodeDataType> where IDDataType: Debug + PartialEq + Eq + Hash + Clone + Copy {
    /// Adds a new node to the graph with the given ID.
    ///
    /// If a node with the given ID already exists, this function will print a warning message and do nothing.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the new node to be added.
    ///
    pub fn add_node(&mut self, id: IDDataType) where IDDataType: Debug + PartialEq + Eq + Hash + Clone + Copy{
        if self.node_data.contains_key(&id) {
            println!("Attempt to add node {:?}, that already exists: ", id);
            return;
        }

        self.nodes.push(id);
        self.node_data.insert(id, NodeDataType::default());
    }

}


impl<IDDataType,NodeDataType> Graph<IDDataType,NodeDataType> where IDDataType: Debug + PartialEq + Eq + Hash + Clone + Copy {
    /// Creates a new, empty graph.
    pub fn new() -> Self {
        Self {
            node_data: FnvHashMap::default(),
            edges: FnvHashMap::default(),
            reverse_edges: FnvHashMap::default(),
            nodes: Vec::new(),
        }
    }

    /// Adds a new node to the graph with the given ID and data.
    ///
    /// If a node with the given ID already exists, this function will print a warning message and do nothing.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the new node to be added.
    /// * `data` - The data to be associated with the new node.
    ///
    pub fn add_node_with_data(&mut self, id: IDDataType, data: NodeDataType) where IDDataType: Debug + PartialEq + Eq + Hash + Clone + Copy{
        if self.node_data.contains_key(&id) {
            println!("Attempt to add node {:?}, that already exists: ", id);
            return;
        }

        self.nodes.push(id);
        self.node_data.insert(id, data);
    }

    /// Add a directed edge from one node to another.
    /// If either node does not exist, this function will do nothing.
    /// If the edge already exists, this function will do nothing.
    /// 
    /// # Arguments
    /// 
    /// * `from` - The ID of the node to add the edge from.
    /// * `to` - The ID of the node to add the edge to.
    /// 
    pub fn add_directed_edge(&mut self, from: IDDataType, to: IDDataType) where IDDataType: Debug + PartialEq + Eq + Hash + Clone + Copy{
        //check if data contains from and to
        if !self.node_data.contains_key(&from) || !self.node_data.contains_key(&to) {
            return;
        }

        //check if edge already exists
        if self.edges.contains_key(&from) && self.edges.get(&from).unwrap().contains(&to) {
            return;
        }

        self.edges.entry(from).or_default().push(to);
        self.reverse_edges.entry(to).or_default().push(from);
    }

    /// Add an undirected edge between two nodes.
    /// If either node does not exist, this function will do nothing.
    /// 
    /// # Arguments
    /// 
    /// * `from` - The ID of the node to add the edge from.
    /// * `to` - The ID of the node to add the edge to.
    /// 
    pub fn add_edge(&mut self, from: IDDataType, to: IDDataType) where IDDataType: Debug + PartialEq + Eq + Hash + Clone + Copy{
        self.add_directed_edge(from, to);
        self.add_directed_edge(to, from);
    }

    /// Get the neighbors of a node.
    /// If the node does not exist, this function will return an empty vector.
    /// 
    /// # Arguments
    /// 
    /// * `id` - The ID of the node to get the neighbors of.
    /// 
    pub fn neighbors(&self, id: IDDataType) -> Vec<IDDataType> where IDDataType: Debug + PartialEq + Eq + Hash + Clone + Copy{
        //check if node exists
        if !self.edges.contains_key(&id) {
            Vec::new()
        } else {
            self.edges[&id].clone()
        }
    }

    /// Get the neighborhood of a node (which includes the node itself).
    /// If the node does not exist, this function will return an empty vector.
    /// 
    /// # Arguments
    /// 
    /// * `id` - The ID of the node to get the neighborhood of.
    /// 
    pub fn neighborhood(&self, id: IDDataType) -> Vec<IDDataType>where IDDataType: Debug + PartialEq + Eq + Hash + Clone + Copy{
        //combine neighbors and self
        let mut neighborhood = self.neighbors(id);
        neighborhood.push(id);
        neighborhood
    }

    /// get the nodes for which the given node is a neighbor.
    /// If the node does not exist, this function will return an empty vector.
    /// 
    /// # Arguments
    /// 
    /// * `id` - The ID of the node to get the reverse neighbors of.
    /// 
    pub fn reverse_neighbors(&self, id: IDDataType) -> &Vec<IDDataType> where IDDataType: Debug + PartialEq + Eq + Hash + Clone + Copy{
        &self.reverse_edges[&id]
    }

    /// checks if the graph is undirected.
    pub fn is_undirected(&self) -> bool {
        for (from, tos) in self.edges.iter() {
            for to in tos {
                if !self.edges.contains_key(to) {
                    return false;
                }
                if !self.edges.get(to).unwrap().contains(from) {
                    return false;
                }
            }
        }
        true
    }

    /// checks if the graph is directed and acyclic.
    pub fn is_directed_acyclic(&self) -> bool {
        //check if graph is directed
        if self.is_undirected() {
            return false;
        }

        //check if graph is acyclic
        for node in self.nodes.iter() {
            if self.is_part_of_a_cycle(*node) {
                return false;
            }
        }
        true
    }

    /// checks if the given node is part of a cycle.
    /// 
    /// # Arguments
    /// 
    /// * `origin` - The ID of the node to check.
    /// 
    fn is_part_of_a_cycle(&self, origin: IDDataType) -> bool {
        /// Potentially check for cycles instead by checking for sources and sinks?
        let mut depth = 0;

        let mut current_layer = self.neighbors(origin);

        while depth < self.nodes.len() {
            let mut next_layer = Vec::new();

            for node in current_layer {
                if node == origin {
                    return true;
                } else {
                    next_layer.append(&mut self.neighbors(node));
                }
            }
            depth += 1;
            current_layer = next_layer;
        }
        false
    }
}

impl<IDDataType,NodeDataType: Default> Default for Graph<IDDataType,NodeDataType>where IDDataType: Debug + PartialEq + Eq + Hash + Clone + Copy{
    fn default() -> Self {
        Self::new()
    }
}

/// generates a grid graph with the given width and height.
pub fn generate_grid_graph<NodeDataType: Default + Send>(
    width: usize,
    height: usize,
) -> Graph<(usize,usize),NodeDataType>{
    let mut g = Graph::new();

    g.node_data = (0..width)
        .flat_map(|x| (0..height).map(move |y| ((x , y), NodeDataType::default())))
        .collect();

    g.nodes = g.node_data.keys().cloned().collect();

    g.edges = g
        .nodes
        .iter()
        .map(|id| {
            let mut tos = Vec::new();
            if id.0 > 0 {
                tos.push((id.0 - 1, id.1));
            }
            if id .0 < width - 1 {
                tos.push((id.0 + 1, id.1));
            }
            if id.1 > 0 {
                tos.push((id.0, id.1 - 1));
            }
            if id.1 < height - 1 {
                tos.push((id.0, id.1 + 1));
            }
            (*id, tos)
        })
        .collect();

    g
}


/// generates a cycle graph with the given number of nodes.
pub fn generate_cycle_graph<NodeDataType: Default + Send>(n: usize) -> Graph<usize,NodeDataType> {
    let mut g = Graph::new();

    //use rayon to create a hashmap of nodes
    g.node_data = (0..n)
        .map(|i| {
            let id = i;
            let node = NodeDataType::default();
            (id, node)
        })
        .collect();

    g.nodes = g.node_data.keys().cloned().collect();

    //use rayon to create a HashMap of edges
    g.edges = g
        .nodes
        .iter()
        .map(|id| {
            let tos = vec![(id + 1) % n, (id + n - 1) % n];
            (*id, tos)
        })
        .collect::<FnvHashMap<usize, Vec<usize>>>();

    g
}

/// generates a hexagonal grid graph with the given width and height.
pub fn generate_hexagonal_grid_graph<IDDataType,NodeDataType: Default + Send>(
    width: usize,
    height: usize,
) -> Graph<usize,NodeDataType>{
    let mut g = Graph::new();

    //use rayon to create a hashmap of nodes
    g.node_data = (0..width * height)
        .map(|i| {
            let id = i;
            let node = NodeDataType::default();
            (id, node)
        })
        .collect();

    g.nodes = g.node_data.keys().cloned().collect();

    //use rayon to create a HashMap of edges
    g.edges = g
        .nodes
        .iter()
        .map(|id| {
            let mut tos = Vec::new();
            if id % width != 0 {
                tos.push(id - 1);
            }
            if id % width != width - 1 {
                tos.push(id + 1);
            }
            if *id >= width {
                tos.push(id - width);
            }
            if *id < width * (height - 1) {
                tos.push(id + width);
            }
            if id % 2 == 0 {
                if *id >= width {
                    tos.push(id - width - 1);
                }
                if *id < width * (height - 1) {
                    tos.push(id + width - 1);
                }
            } else {
                if *id >= width {
                    tos.push(id - width + 1);
                }
                if *id < width * (height - 1) {
                    tos.push(id + width + 1);
                }
            }
            (*id, tos)
        })
        .collect::<FnvHashMap<usize, Vec<usize>>>();

    g
}


/// counts the number of paths from the start node to the end node.
pub fn count_paths<IDDataType, NodeDataType>(graph: &Graph<IDDataType, NodeDataType>, start: &IDDataType, end: &IDDataType) -> usize
where
    IDDataType: Debug + PartialEq + Eq + Hash + Clone + Copy,
{
    // function body

    assert!(graph.nodes.contains(start), "graph does not contain start");
    assert!(graph.nodes.contains(end), "graph does not contain end");
    assert!(graph.is_directed_acyclic(), "graph is not directed acyclic");


    let mut paths = 0;

    let reverse_neighbors = graph.reverse_neighbors(*end);
    for reverse_neighbor in reverse_neighbors {
        if reverse_neighbor == start {
            paths += 1;
        } else {
            paths += count_paths(graph, start, reverse_neighbor);
        }
    }

    paths
}
