use std::io;
use std::fs::{self, DirEntry};
use std::path::Path;
use std::collections::HashMap;

// DI Parser, TweetGenerator
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
        // println!("{}", word.to_string());
        self.graph.add(word.to_string(), last_word);
        last_word = Some(word.to_string());
      }
    }
    Ok(())
  }

  fn generate_tweet(self: &Self) -> String {
    return String::from("");
  }

  pub fn create_tweets(&mut self, dir: &Path, number: i32) -> Vec<String> {
    // should I do something with the io::Result here?
    self.parse_in(dir);

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
  map: HashMap<String, Node>,
  entry_words: Vec<String>, // storing capitalized words
}

impl Graph {
  fn number_of_nodes(self) -> i32 {
    return self.map.len().try_into().unwrap();
  }

  fn add(&mut self, word: String, last_word: Option<String>) -> () {
    // check to see if we already have this word
    // if we do, we change the edges on it - add new one, or strengthen existing one
    // if we don't, we add it to the map
    self.map.insert(word, Node{});
  }

  pub fn new() -> Graph {
    return Graph {
      map: HashMap::new(),
      entry_words: Vec::new(),
    };
  }
}

// we need to store a weighted index (the 'strength' of an edge) for probabilistic sampling
struct Node {

}

impl Node {
  // randomly picks from weighted edges and returns pointer to next node
  fn next(self) -> Node {
    return Node{};
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  #[ignore]
  fn add() {
    let mut graph = Graph::new();
    graph.add(String::from("foo"), None);
    assert_eq!(graph.number_of_nodes(), 1);
  }

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

  // #[test]
  // fn parse_in() {
  //   let test_path: &Path = Path::new("./txt");

  //   let response = MarkovChain::parse_in(test_path);
  // }

  #[test]
  fn create_a_tweet() {
    let test_path: &Path = Path::new("./txt");
    let mut mchain = MarkovChain::new();

    let response = mchain.create_tweets(test_path, 1);
    assert_eq!(response[0], "implement me pls");
  }
}