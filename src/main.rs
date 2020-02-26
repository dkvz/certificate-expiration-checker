use std::env;
//mod local_config;
//pub use local_config::*;

fn main() {
  let args: Vec<String> = env::args().collect();

  let pos = args.iter().position(|i| i == "-f")
    .expect("Please provide a config file with the -f option");

  /*if args.len() < 3 {
    // Missing the file argument...
    panic!("Please provide a filename for the config file");
  }*/

  // Let's use the get() method on vectors:
  let filename = args.get(pos + 1)
    .expect("Please provide a filename for the config file");

  println!("Found filename {}", filename);
}
