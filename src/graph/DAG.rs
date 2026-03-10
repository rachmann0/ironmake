use crate::graph::artifact::{Artifact};

// ! Directed Acyclic Graph (DAG)

// pub struct Rule {
// }

pub struct Graph{ // edges directed to same target
    pub nodes: Vec<Artifact>,
    // pub edges: Vec<Rule>
}

impl Graph {
    pub fn new(nodes:Vec<Artifact>) -> Graph {
        Graph { nodes }
    }
}
