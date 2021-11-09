/*
"Erowid Coin" is just a markov chain generator for tweeting about the hellish crossection of erowid trip
reports + cryptocurrency - it's build using local text files. It's structed as such, using dependency injection:

MarkovChain -> Parser
          |--> TweetGenerator
*/

use std::env;

pub mod markov_chain;

fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() < 2 {
    println!("usage: erowid_coin <text directory>");
    return;
  }

  let directory = &args[1];

  let mchain = markov_chain::MarkovChain {};
}
