use std::fs::{self, DirEntry};
use std::path::Path;
use std::collections::HashMap;

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
// I might end up duplicating this to allow for random sampling faster, I think Rust is O(n) for randomly sampling
// from a HashMap, but I only need to do that once for determining the first word in a tweet.
struct Graph {
  map: HashMap<String, Node>,
}

impl Graph {
  pub fn new() -> Graph {
    return Graph {
      map: HashMap::new(),
    };
  }
}

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
  fn parse_in() {
    let test_path: &Path = Path::new("./txt");

    let response = MarkovChain::parse_in(test_path);

    // assert_eq!(result, 5);
  }
}


struct TweetGenerator {

}

impl TweetGenerator {

}