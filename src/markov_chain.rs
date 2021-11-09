use std::fs::{self, DirEntry};
use std::path::Path;
use std::collections::HashMap;
use std::error::Error;

// DI Parser, TweetGenerator
// contains a graph structure
pub struct MarkovChain {
  
}

impl MarkovChain {
  fn parse_in(dir: &Path) -> Option<&Graph>{

    None
  }

  pub fn create_tweets(self: &Self, dir: &Path, number: i32) -> &str {
    return "this is a tweet";
  }

  pub fn new() -> MarkovChain {
    return MarkovChain {};
  }
}

// we mostly care about fast lookups for adding new nodes / modifying edges for existing ones.
// I might end up duplicating this to allow for faster random sampling, I think Rust is O(n) for randomly sampling
// from a HashMap, but I only need to do that once for determining the first word in a tweet.
#[derive(Clone)]
struct Graph {
  map: HashMap<String, Node>,
  entryWords: Vec<String>, // storing capitalized words 
}

impl Graph {
  fn number_of_nodes(self) -> i32 {
    return self.map.len().try_into().unwrap();
  }

  fn add(&mut self, word: String) -> () {
    self.map.insert(word, Node{});
  }

  pub fn new() -> Graph {
    return Graph {
      map: HashMap::new(),
      entryWords: Vec::new(),
    };
  }
}

// we need to store a weighted index (the 'strength' of an edge) for probabilistic sampling
#[derive(Clone)]
struct Node {

}

// struct Edge {

// }

// struct Parser {

// }

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn add() {
    let mut graph = Graph::new();
    graph.add(String::from("foo"));
    assert_eq!(graph.number_of_nodes(), 1);
  }

  fn add_duplicate() {
    let graph = Graph::new();
    // graph.add
  }

  #[test]
  fn parse_in() {
    let test_path: &Path = Path::new("./txt");

    let response = MarkovChain::parse_in(test_path);
  }
}


// struct TweetGenerator {

// }

// impl TweetGenerator {

// }