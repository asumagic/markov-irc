use std::collections::HashMap;

use anyhow::*;
use rand::{thread_rng, Rng};

struct Node {
    name: String,
    successors: Vec<String>,
}

impl Node {
    fn new(name: String) -> Self {
        Self {
            name,
            successors: Vec::new(),
        }
    }

    fn add_succ(&mut self, succ: String) {
        self.successors.push(succ);
    }
}

pub struct Markov {
    nodes: HashMap<String, Node>,
}

impl Markov {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }

    pub fn insert(&mut self, name: String, pred: String) {
        self.nodes
            .entry(name.clone())
            .or_insert_with(|| Node::new(name.clone()));

        if !pred.is_empty() {
            self.nodes.get_mut(&pred).unwrap().add_succ(name);
        }
    }

    pub fn random_chain(&self, hook_word: &str) -> Result<Vec<String>> {
        let mut rng = thread_rng();

        let mut words = Vec::new();
        let mut current = self
            .nodes
            .get(hook_word)
            .ok_or_else(|| anyhow!(r#"hook word "{}" not found for this user"#, hook_word))?;

        loop {
            if current.successors.is_empty() || current.name.is_empty() {
                break;
            }

            let random_succ = &current.successors[rng.gen_range(0, current.successors.len())];

            words.push(current.name.clone());

            current = &self.nodes[random_succ];
        }

        Ok(words)
    }
}
