use crate::ds::artifact::{Artifact};

// ! Directed Acyclic Graph (DAG)

#[derive(Debug, Clone)]
pub struct Rule {
    pub dependancy_indexes: Vec<usize>,
    pub target_index: usize
}

impl Rule {
    pub fn new(dependancy_indexes: Vec<usize>, target_index: usize)-> Rule{
        Rule {
            dependancy_indexes,
            target_index
        }
    }
}

pub struct Graph{ // edges directed to same target
    pub nodes: Vec<Artifact>,
    pub edges: Vec<Rule>,
}

impl Graph {
    pub fn new(nodes:Vec<Artifact>, edges:Vec<Rule>) -> Graph {
        Graph {
            nodes,
            edges,
        }
    }
}
