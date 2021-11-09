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

  pub fn create_tweets(self: &Self, directory: &str, number: i32) -> &str {
    return "this is a tweet";
  }

  pub fn new() -> MarkovChain {
    return MarkovChain {};
  }
}

struct Graph {

}

// struct Node {

// }

// struct Edge {

// }

// struct Parser {

// }

// impl Parser {
//   // fn 

//   pub fn parse_in(dir: &Path) {
    
//   }

//   pub fn new() -> Parser {
//     return Parser {};
//   }
  // }

#[cfg(test)]
mod tests {
  use super::*;
  // use Parser;

  #[test]
  fn parsing() {
    let test_path: &Path = Path::new("./txt");

    // let mchain = MarkovChain::new();
    let response = MarkovChain::parse_in(test_path);

    // assert_eq!(result, 5);
  }
}


struct TweetGenerator {

}

impl TweetGenerator {

}