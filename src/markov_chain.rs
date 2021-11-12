use std::{io, fs};
use std::path::Path;
use std::collections::HashMap;
use rand::Rng;
use rand::seq::SliceRandom;
use regex::Regex;

// contains a graph structure
pub struct MarkovChain {
  graph: Graph,
}

impl MarkovChain {
  // builds our graph
  fn parse_in(&mut self, dir: &Path) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
      let entry = entry?;
      let path = entry.path();
      let contents = fs::read_to_string(path)?;
      let contents = contents.split_whitespace();

      let mut last_word: Option<String> = None;

      for word in contents {
        self.graph.add(word.to_string(), last_word);
        last_word = Some(word.to_string());
      }
    }
    Ok(())
  }

  fn generate_tweet(&mut self) -> String {
    return self.graph.generate_tweet();
  }

  pub fn create_tweets(&mut self, dir: &Path, number: i32) -> Vec<String> {
    self.parse_in(dir).unwrap();

    let mut vec = Vec::new();

    for _ in 0..number {
      vec.push(self.generate_tweet());
    }

    return vec;
  }

  pub fn new() -> MarkovChain {
    return MarkovChain {
      graph: Graph::new(),
    };
  }
}

// we mostly care about fast lookups for adding new nodes / modifying edges for existing ones.
// I might end up duplicating this to allow for faster random sampling, I think Rust is O(n) for randomly sampling
// from a HashMap, but I only need to do that once for determining the first word in a tweet.
struct Graph {
  nodes: HashMap<String, Node>,
  entry_words: Vec<String>, // storing capitalized words
}

impl Graph {
  fn generate_tweet(&mut self) -> String {
    let mut tweet = self.random_entry_word();

    let re = Regex::new(".*[!|.|?]$").unwrap();
    // while !re.is_match(&tweet) {
    // }

    return tweet;
  }

  fn random_entry_word(&mut self) -> String {
    let mut rng = rand::thread_rng();
    let word = self.entry_words.choose(&mut rng).unwrap();
    
    return word.to_string();
  }

  // fn next(self, word: String) -> String {

  // }

  fn add(&mut self, word: String, last_word: Option<String>) -> () {
    let uppercase = Regex::new(r"\A[A-Z]\w*").unwrap();

    if !self.nodes.contains_key(&word) {
      self.nodes.insert(word.clone(), Node::new());

      if uppercase.is_match(word.as_str()) {
        self.entry_words.push(word.clone());
      }
    }

    if let Some(last_word) = last_word {
      let last_node = self.nodes.get_mut(&last_word).unwrap();
      last_node.strengthen_edge(word);
    }
  }

  pub fn new() -> Graph {
    return Graph {
      nodes: HashMap::new(),
      entry_words: Vec::new(),
    };
  }
}

// we need to store a weighted index (the 'strength' of an edge) for probabilistic sampling
struct Node {
  // can we have it store a reference to the next node? Would be way nicer than having the graph need to reach in for this ("tell, don't ask")
  edges: HashMap<String, i32>,
  sum: i32,
}

impl Node {
  // randomly picks from weighted edges
  // there's actually a way to do weighted randomization with rand::distributions::WeightedIndex, might want to use that instead
  fn next(self) -> String {
    let mut number = rand::thread_rng().gen_range(1..=self.sum);

    for (word, weight) in self.edges {
      number -= weight;

      if number <= 0 {
        return word;
      }
    }

    panic!("the edge weights do not match the sum");
  }

  // edges are node -> weight
  fn strengthen_edge(&mut self, next: String) -> () {
    let weight = self.edges.entry(next.clone()).or_insert(0);
    *weight += 1;
    self.sum += 1;
  }

  pub fn new() -> Node {
    return Node {
      edges: HashMap::new(),
      sum: 0,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  // Graph tests - these are more of an internal implementation detail for MarkovChain, so they can prob be deleted later

  // #[test]
  // #[ignore]
  // fn add() {
  //   let mut graph = Graph::new();
  //   graph.add(String::from("foo"), None);
  //   assert_eq!(graph.number_of_nodes(), 1);
  // }

  // fn add_duplicate() {
  //   let mut graph = Graph::new();
  //   graph.add(String::from("foo"));
  //   let node graph.add(String::from("foo"));
  //   assert_eq!(graph.number_of_nodes(), 1);
  // }

  #[test]
  #[ignore]
  fn add_strengthens_edges() {
  }

  // MarkovChain tests

  #[test]
  fn create_a_tweet() {
    let test_path: &Path = Path::new("./txt");
    let mut mchain = MarkovChain::new();

    let response = mchain.create_tweets(test_path, 1);
    assert_eq!(response[0], "implement me pls");
  }
}