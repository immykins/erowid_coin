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
  let mut num_tweets: i32 = 1;

  if args.len() < 2 {
    println!("usage: erowid_coin <text directory> <number of tweets>");
    return;
  }

  if args.len() == 3 {
    // handle errors here lol
    let integer = &args[2].parse::<i32>();
    let num_tweets = match integer {
      Ok(val) => val,
      Err(error) => {
        println!("could not parse number of tweets: {}", error);
        return;
      },
    };
  }

  let directory = &args[1];

  let mchain = MarkovChain::new();
  let tweet = mchain.create_tweets(directory, num_tweets);

  println!("{}", tweet);
}
