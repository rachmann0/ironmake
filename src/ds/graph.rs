use crate::ds::artifact::{Artifact};

// ! Directed Acyclic Graph (DAG)

pub struct Graph{ // edges directed to same target
    pub nodes: Vec<Artifact>,
}

impl Graph {
    pub fn new(nodes:Vec<Artifact>) -> Self {
        Self {
            nodes,
        }
    }
}
