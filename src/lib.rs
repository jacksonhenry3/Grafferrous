use fnv::FnvHashMap as HashMap;
use rayon::prelude::*;
use std::marker::Send;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct ID(pub usize);

#[derive(Debug)]
#[allow(dead_code)]
pub struct Node<NodeDataType> {
    pub id: ID,
    pub data: NodeDataType,
}

#[derive(Debug)]
pub struct GraphData<NodeDataType> {
    pub nodes: HashMap<ID, Node<NodeDataType>>,
    pub edges: HashMap<ID, Vec<ID>>,
}

impl<NodeDataType> GraphData<NodeDataType> {
    fn new() -> Self {
        Self {
            nodes: HashMap::default(),
            edges: HashMap::default(),
        }
    }
}

pub struct Graph<NodeDataType> {
    pub data: GraphData<NodeDataType>,
    pub keys: Vec<ID>,
}

impl<NodeDataType: Default> Graph<NodeDataType> {
    pub fn new() -> Self {
        Self {
            data: GraphData::new(),
            keys: Vec::new(),
        }
    }

    pub fn add_node(&mut self, id: ID) {
        //check if data contains id
        if self.data.nodes.contains_key(&id) {
            return;
        }

        self.keys.push(id);
        self.data.nodes.insert(
            id,
            Node {
                id,
                data: NodeDataType::default(),
            },
        );
    }

    pub fn add_directed_edge(&mut self, from: ID, to: ID) {
        //check if data contains from and to
        if !self.data.nodes.contains_key(&from) || !self.data.nodes.contains_key(&to) {
            return;
        }

        //check if edge already exists
        if self.data.edges.contains_key(&from) && self.data.edges.get(&from).unwrap().contains(&to)
        {
            return;
        }

        self.data.edges.entry(from).or_default().push(to);
    }

    pub fn add_edge(&mut self, from: ID, to: ID) {
        self.add_directed_edge(from, to);
        self.add_directed_edge(to, from);
    }

    pub fn neighbors(&self, id:ID) -> &Vec<ID> {
        &self.data.edges[&id]
    }

    pub fn neighborhood(&self, id:ID) -> Vec<ID> {
        //combine neighbors and self
        let mut neighborhood = self.neighbors(id).clone();
        neighborhood.push(id);
        neighborhood
    }
    pub fn is_undirected(&self) -> bool {
        for (from, tos) in self.data.edges.iter() {
            for to in tos {
                if !self.data.edges.contains_key(to) {
                    return false;
                }
                if !self.data.edges.get(to).unwrap().contains(from) {
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
    g.data.nodes = (0..width * height)
        .into_par_iter()
        .map(|i| {
            let id = ID(i);
            let node = Node {
                id,
                data: NodeDataType::default(),
            };
            (id, node)
        })
        .collect();

    g.keys = g.data.nodes.keys().cloned().collect();

    //use rayon to generate a hasmap of edges
    g.data.edges = (0..width * height)
        .into_par_iter()
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
    g.data.nodes = (0..n)
        .into_par_iter()
        .map(|i| {
            let id = ID(i);
            let node = Node {
                id,
                data: NodeDataType::default(),
            };
            (id, node)
        })
        .collect();

    g.keys = g.data.nodes.keys().cloned().collect();

    //use rayon to create a HashMap of edges
    g.data.edges = g
        .keys
        .par_iter()
        .map(|id| {
            let tos = vec![ID((id.0 + 1) % n), ID((id.0 + n - 1) % n)];
            (*id, tos)
        })
        .collect::<HashMap<ID, Vec<ID>>>();

    g
}

pub fn generate_hexagonal_grid_graph<NodeDataType: Default + Send>(
    width: usize,
    height: usize,
) -> Graph<NodeDataType> {
    let mut g = Graph::new();

    //use rayon to create a hashmap of nodes
    g.data.nodes = (0..width * height)
        .into_par_iter()
        .map(|i| {
            let id = ID(i);
            let node = Node {
                id,
                data: NodeDataType::default(),
            };
            (id, node)
        })
        .collect();

    g.keys = g.data.nodes.keys().cloned().collect();

    //use rayon to create a HashMap of edges
    g.data.edges = g
        .keys
        .par_iter()
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
        .collect::<HashMap<ID, Vec<ID>>>();

    g
}
