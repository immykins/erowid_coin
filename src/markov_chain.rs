use std::io;
use std::fs;
use std::path::Path;
use std::collections::HashMap;

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
  // entry_words: Vec<String>, // storing capitalized words
}

impl Graph {
  fn number_of_nodes(self) -> i32 {
    return self.nodes.len().try_into().unwrap();
  }

  fn add(&mut self, word: String, last_word: Option<String>) -> () {
    if !self.nodes.contains_key(&word) {
      self.nodes.insert(word, Node::new());
      return;
    }

    if last_word.is_some() {
      let last_word = last_word.unwrap();
      let mut last_node = self.nodes[&last_word];
      let current_word = self.nodes[&word];

      last_node.strengthen_edge(current_word);
      // self.map.insert(word, Node{});
    }
    // if we do, we change the edges on the PREVIOUS - add new one, or strengthen existing one
  }

  pub fn new() -> Graph {
    return Graph {
      nodes: HashMap::new(),
      // entry_words: Vec::new(),
    };
  }
}

// we need to store a weighted index (the 'strength' of an edge) for probabilistic sampling
struct Node {
  // can we have it store a reference to the next node? Would be way nicer than having the graph need to reach in for this ("tell, don't ask")
  edges: HashMap<String, i32>, // edge is the next 
}

impl Node {
  // randomly picks from weighted edges and returns pointer to next node
  // IDEA: instead of (or in addition to) doing this, would be really cool to do our own iterator implementation... maybe use collect and convert it into a string?
  fn next(self) -> Node {
    return Node::new();
  }

  fn strengthen_edge(&mut self, next: Node) -> () {

  }

  pub fn new() -> Node {
    return Node {
      edges: HashMap::new(),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  // Graph tests - these are more of an internal implementation detail for MarkovChain, so they can prob be deleted later

  #[test]
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

  // MarkovChain tests

  #[test]
  fn create_a_tweet() {
    let test_path: &Path = Path::new("./txt");
    let mut mchain = MarkovChain::new();

    let response = mchain.create_tweets(test_path, 1);
    assert_eq!(response[0], "implement me pls");
  }
}