#![allow(unused)]

use fnv::FnvHashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct ID(pub usize);

pub struct Graph<NodeDataType> {
    pub node_data: FnvHashMap<ID, NodeDataType>,
    pub edges: FnvHashMap<ID, Vec<ID>>,
    pub nodes: Vec<ID>,
}

impl<NodeDataType: Default> Graph<NodeDataType> {
    pub fn new() -> Self {
        Self {
            node_data: FnvHashMap::default(),
            edges: FnvHashMap::default(),
            nodes: Vec::new(),
        }
    }

    pub fn add_node(&mut self, id: ID) {
        if self.node_data.contains_key(&id) {
            println!("Attempt to add node {:?}, that already exists: ", id);
            return;
        }

        self.nodes.push(id);
        self.node_data.insert(id, NodeDataType::default());
    }

    pub fn add_directed_edge(&mut self, from: ID, to: ID) {
        //check if data contains from and to
        if !self.node_data.contains_key(&from) || !self.node_data.contains_key(&to) {
            return;
        }

        //check if edge already exists
        if self.edges.contains_key(&from) && self.edges.get(&from).unwrap().contains(&to) {
            return;
        }

        self.edges.entry(from).or_default().push(to);
    }

    pub fn add_edge(&mut self, from: ID, to: ID) {
        self.add_directed_edge(from, to);
        self.add_directed_edge(to, from);
    }

    pub fn neighbors(&self, id: ID) -> &Vec<ID> {
        &self.edges[&id]
    }

    pub fn neighborhood(&self, id: ID) -> Vec<ID> {
        //combine neighbors and self
        let mut neighborhood = self.neighbors(id).clone();
        neighborhood.push(id);
        neighborhood
    }

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
}

impl<NodeDataType: Default> Default for Graph<NodeDataType> {
    fn default() -> Self {
        Self::new()
    }
}

pub fn generate_grid_graph<NodeDataType: Default + Send>(
    width: usize,
    height: usize,
) -> Graph<NodeDataType> {
    let mut g = Graph::new();
    //use rayon to generate a HashMap of nodes
    g.node_data = (0..width * height)
        .into_iter()
        .map(|i| (ID(i), NodeDataType::default()))
        .collect();

    g.nodes = g.node_data.keys().cloned().collect();

    g.edges = (0..width * height)
        .into_iter()
        .map(|i| {
            let id = ID(i);
            let mut tos = Vec::new();
            if i % width != 0 {
                tos.push(ID(i - 1));
            }
            if i % width != width - 1 {
                tos.push(ID(i + 1));
            }
            if i >= width {
                tos.push(ID(i - width));
            }
            if i < width * (height - 1) {
                tos.push(ID(i + width));
            }
            (id, tos)
        })
        .collect();

    g
}

pub fn generate_cycle_graph<NodeDataType: Default + Send>(n: usize) -> Graph<NodeDataType> {
    let mut g = Graph::new();

    //use rayon to create a hashmap of nodes
    g.node_data = (0..n)
        .into_iter()
        .map(|i| {
            let id = ID(i);
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
            let tos = vec![ID((id.0 + 1) % n), ID((id.0 + n - 1) % n)];
            (*id, tos)
        })
        .collect::<FnvHashMap<ID, Vec<ID>>>();

    g
}

pub fn generate_hexagonal_grid_graph<NodeDataType: Default + Send>(
    width: usize,
    height: usize,
) -> Graph<NodeDataType> {
    let mut g = Graph::new();

    //use rayon to create a hashmap of nodes
    g.node_data = (0..width * height)
        .into_iter()
        .map(|i| {
            let id = ID(i);
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
            if id.0 % width != 0 {
                tos.push(ID(id.0 - 1));
            }
            if id.0 % width != width - 1 {
                tos.push(ID(id.0 + 1));
            }
            if id.0 >= width {
                tos.push(ID(id.0 - width));
            }
            if id.0 < width * (height - 1) {
                tos.push(ID(id.0 + width));
            }
            if id.0 % 2 == 0 {
                if id.0 >= width {
                    tos.push(ID(id.0 - width - 1));
                }
                if id.0 < width * (height - 1) {
                    tos.push(ID(id.0 + width - 1));
                }
            } else {
                if id.0 >= width {
                    tos.push(ID(id.0 - width + 1));
                }
                if id.0 < width * (height - 1) {
                    tos.push(ID(id.0 + width + 1));
                }
            }
            (*id, tos)
        })
        .collect::<FnvHashMap<ID, Vec<ID>>>();

    g
}
