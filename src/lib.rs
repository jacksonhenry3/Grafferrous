use core::hash::Hash;
use fnv::FnvHashMap;

use std::fmt::Debug;

#[derive(Debug, PartialEq, Eq, Clone)]
/// A graph data structure with nodes of type `NodeDataType` and edges between them.
pub struct Graph<IDDataType, NodeDataType>
where
    IDDataType: Debug + PartialEq + Eq + Hash + Clone + Copy,
{
    /// A map from node IDs to their associated data.
    pub node_data: FnvHashMap<IDDataType, NodeDataType>,
    /// A map from node IDs to a vector of their outgoing edges.
    pub edges: FnvHashMap<IDDataType, Vec<IDDataType>>,
    /// A map from node IDs to a vector of their incoming edges.
    pub reverse_edges: FnvHashMap<IDDataType, Vec<IDDataType>>,
    /// A vector of all node IDs in the graph.
    pub nodes: Vec<IDDataType>,
}

impl<IDDataType, NodeDataType: Default> Graph<IDDataType, NodeDataType>
where
    IDDataType: Debug + PartialEq + Eq + Hash + Clone + Copy,
{
    /// Creates a new, empty graph.
    pub fn new() -> Self {
        Self {
            node_data: FnvHashMap::default(),
            edges: FnvHashMap::default(),
            reverse_edges: FnvHashMap::default(),
            nodes: Vec::new(),
        }
    }

    // graph from edges
    pub fn from_edges(edges: &[(IDDataType, IDDataType)]) -> Self {
        let mut graph = Self::new();

        for (from, to) in edges {
            graph.add_directed_edge(*from, *to);
        }
        graph
    }

    /// Adds a new node to the graph with the given ID.
    ///
    /// If a node with the given ID already exists, this function will print a warning message and do nothing.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the new node to be added.
    ///
    pub fn add_node(&mut self, id: IDDataType) {
        //use add_node_with_data
        self.add_node_with_data(id, NodeDataType::default());
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
    pub fn add_node_with_data(&mut self, id: IDDataType, data: NodeDataType) {
        if self.node_data.contains_key(&id) {
            println!("Attempt to add node {:?}, that already exists: ", id);
            return;
        }

        self.nodes.push(id);
        self.edges.insert(id, Vec::new());
        self.reverse_edges.insert(id, Vec::new());
        self.node_data.insert(id, data);
    }

    /// Add a directed edge from one node to another.
    /// If either node does not exist, this function will add them.
    /// If the edge already exists, this function will do nothing.
    ///
    /// # Arguments
    ///
    /// * `from` - The ID of the node to add the edge from.
    /// * `to` - The ID of the node to add the edge to.
    ///
    pub fn add_directed_edge(&mut self, from: IDDataType, to: IDDataType) {
        // if the node does not exist, add it
        if !self.node_data.contains_key(&from) {
            // println!("Attempt to add edge from {:?} to {:?}, but {:?} does not exist. Adding {:?} to the graph.", from, to, from, from);
            self.add_node(from);
        }

        if !self.node_data.contains_key(&to) {
            // println!("Attempt to add edge from {:?} to {:?}, but {:?} does not exist. Adding {:?} to the graph.", from, to, to, to);
            self.add_node(to);
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
    pub fn add_edge(&mut self, from: IDDataType, to: IDDataType) {
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
    pub fn neighbors(&self, id: IDDataType) -> Vec<IDDataType> {
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
    pub fn neighborhood(&self, id: IDDataType) -> Vec<IDDataType> {
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
    pub fn reverse_neighbors(&self, id: IDDataType) -> &Vec<IDDataType> {
        &self.reverse_edges[&id]
    }

    ///edge tuples
    /// get the edges of the graph as a vector of tuples.
    ///
    ///
    pub fn edge_tuples(&self) -> Vec<(IDDataType, IDDataType)> {
        let mut edge_tuples = Vec::new();
        for (from, tos) in self.edges.iter() {
            for to in tos {
                edge_tuples.push((*from, *to));
            }
        }
        edge_tuples
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
        // Potentially check for cycles instead by checking for sources and sinks?
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

impl<IDDataType, NodeDataType: Default> Default for Graph<IDDataType, NodeDataType>
where
    IDDataType: Debug + PartialEq + Eq + Hash + Clone + Copy,
{
    fn default() -> Self {
        Self::new()
    }
}

//macro to create a graph from a list of edges
#[macro_export]
macro_rules! graph {
    ($($from:expr => $to:expr),*) => {
        {
            let mut g = Graph::new();

            //just add the first node

            $(

                g.add_directed_edge($from, $to);
            )*
            g
        }
    };

    ($($from:expr ; $to:expr),*) => {
        {
            let mut g = Graph::new();

            //just add the first node

            $(

                g.add_edge($from, $to);
            )*
            g
        }
    };


    (($($id:expr,$data:expr),*),($($from:expr => $to:expr),*)) => {
        {
            let mut g = Graph::new();

            $(
                $(
                    g.add_node_with_data($id,$data);
                )*

                $(
                    g.add_directed_edge($from, $to);
                )*
            )*
            g
        }
    };
}

/// generates a grid graph with the given width and height.
pub fn generate_grid_graph<NodeDataType: Default + Send>(
    width: usize,
    height: usize,
) -> Graph<(usize, usize), NodeDataType> {
    let mut g = Graph::new();

    g.node_data = (0..width)
        .flat_map(|x| (0..height).map(move |y| ((x, y), NodeDataType::default())))
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
            if id.0 < width - 1 {
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
pub fn generate_cycle_graph<NodeDataType: Default + Send>(n: usize) -> Graph<usize, NodeDataType> {
    let mut g = Graph::new();

    //create a hashmap of nodes
    g.node_data = (0..n)
        .map(|i| {
            let id = i;
            let node = NodeDataType::default();
            (id, node)
        })
        .collect();

    g.nodes = g.node_data.keys().cloned().collect();

    //create a HashMap of edges
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

/// generates a random graph with the given number of nodes and edge probability.
///
/// # Arguments
///
/// * `n` - The number of nodes in the graph.
/// * `p` - The probability of an edge between two nodes.
///
pub fn generate_random_graph<NodeDataType: Default + Send>(
    n: usize,
    p: f64,
) -> Graph<usize, NodeDataType> {
    let mut g = Graph::new();

    //create a hashmap of nodes
    g.node_data = (0..n)
        .map(|i| {
            let id = i;
            let node = NodeDataType::default();
            (id, node)
        })
        .collect();

    g.nodes = g.node_data.keys().cloned().collect();

    //create a HashMap of edges
    g.edges = g
        .nodes
        .iter()
        .map(|id| {
            let mut tos = Vec::new();
            for to in 0..n {
                if rand::random::<f64>() < p {
                    tos.push(to);
                }
            }
            (*id, tos)
        })
        .collect::<FnvHashMap<usize, Vec<usize>>>();

    g
}

//consider adding triangular grid and hexagonal grid

/// counts the number of paths from the start node to the end node.
pub fn count_paths<IDDataType, NodeDataType: Default>(
    graph: &Graph<IDDataType, NodeDataType>,
    start: &IDDataType,
    end: &IDDataType,
) -> usize
where
    IDDataType: Debug + PartialEq + Eq + Hash + Clone + Copy,
{
    // function body

    assert!(graph.nodes.contains(start), "graph does not contain start");
    assert!(graph.nodes.contains(end), "graph does not contain end");
    assert!(graph.is_directed_acyclic(), "graph is not directed acyclic");

    // base case
    if start == end {
        return 1;
    }

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
