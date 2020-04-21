pub struct AdjacencyList {
    dimension: usize,
    list: Vec<Vec<usize>>,
}

impl AdjacencyList {
    pub fn new() -> AdjacencyList {
        AdjacencyList {
            dimension: 0,
            list: vec![vec![]],
        }
    }

    pub fn has_edge(edge: usize) -> bool {
        true
    }

    pub fn insert(&mut self, edge: usize, new: usize) {
        match self.list.get_mut(edge) {
            _ => {}
        }
    }
}
