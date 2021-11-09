/*
"Erowid Coin" is just a markov chain generator for tweeting about the hellish crossection of erowid trip
reports + cryptocurrency - it's build using local text files. It's structed as such, using dependency injection:

MarkovChain -> Parser
          |--> TweetGenerator
*/

pub mod markov_chain;

use std::env;
use markov_chain::MarkovChain;

fn main() {
  let args: Vec<String> = env::args().collect();
  let numTweets = 1;

  if args.len() < 2 {
    println!("usage: erowid_coin <text directory>");
    return;
  }

  if args.len() === 3 {
    numTweets = &args[2];
  }

  let directory = &args[1];

  let mchain = MarkovChain::new();
  let tweet = mchain.create_tweets(directory, numTweets);

  println!("{}", tweet);
}
