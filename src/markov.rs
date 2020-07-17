use fxhash::FxHashMap;

use anyhow::*;
use rand::{thread_rng, Rng};

type WordIndex = u32;

const WORD_SENTENCE_BEGIN: WordIndex = 0;
const WORD_SENTENCE_END: WordIndex = 1;
const WORD_USER_BEGIN: WordIndex = 2;

struct Node {
    succs: Vec<Vec<WordIndex>>,
}

impl Node {
    fn new() -> Self {
        Self { succs: Vec::new() }
    }
}

pub struct Markov {
    word_matches: FxHashMap<String, WordIndex>,
    nodes: FxHashMap<WordIndex, Node>,
    latest_user_word: WordIndex,
}

impl Markov {
    pub fn new() -> Self {
        Self {
            word_matches: FxHashMap::default(),
            nodes: FxHashMap::default(),
            latest_user_word: WORD_USER_BEGIN,
        }
    }

    fn get_or_insert_word_index(&mut self, word: String) -> WordIndex {
        // work around the borrow checker making me sad
        let mut latest_word = self.latest_user_word;

        let entry = *self.word_matches.entry(word).or_insert_with(|| {
            latest_word += 1;
            latest_word - 1
        });

        self.latest_user_word = latest_word;

        entry
    }

    fn get_word_index(&self, word: &str) -> Option<&WordIndex> {
        self.word_matches.get(word)
    }

    fn get_word_from_index(&self, word_index: WordIndex) -> Option<&str> {
        // TODO: not a O(n) lookup
        self.word_matches
            .iter()
            .find(|(_k, v)| **v == word_index)
            .map(|(k, _v)| k.as_ref())
    }

    pub fn insert_word(&mut self, word: WordIndex, mut succs: &[WordIndex]) {
        let current_entry = self.nodes.entry(word).or_insert_with(Node::new);

        if !succs.is_empty() {
            current_entry.succs.push(Vec::from(succs));
            self.insert_word(succs[0], &succs[1..]);
        }
    }

    pub fn insert_sentence(&mut self, sentence: Vec<String>) {
        let mut word_indices = Vec::new();

        word_indices.push(WORD_SENTENCE_BEGIN);
        word_indices.extend(
            sentence
                .iter()
                .map(|s| self.get_or_insert_word_index(s.to_string())),
        );
        word_indices.push(WORD_SENTENCE_END);

        for i in 0..sentence.len() {
            self.insert_word(word_indices[i], &word_indices[i + 1..]);
        }
    }

    pub fn random_chain(&self) -> Result<Vec<&str>> {
        let mut rng = thread_rng();

        let mut words = Vec::new();

        let mut current_index = 0;
        let mut current_node = self
            .nodes
            .get(&WORD_SENTENCE_BEGIN)
            .ok_or_else(|| anyhow!("sentence begin node not found, dataset empty?"))?;

        loop {
            if current_node.succs.is_empty() || current_index == WORD_SENTENCE_END {
                break;
            }

            let random_succ = &current_node.succs[rng.gen_range(0, current_node.succs.len())];

            current_index = random_succ[0];
            current_node = &self.nodes[&current_index];

            if current_index >= WORD_USER_BEGIN {
                words.push(self.get_word_from_index(current_index).unwrap());
            }
        }

        Ok(words)
    }
}
