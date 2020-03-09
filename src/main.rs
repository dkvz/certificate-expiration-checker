use std::env;
/*mod local_config;
use local_config::ConfigFile;*/
use certexpchecker::{ConfigFile, run};

fn main() {
  // Collect command line args but skip the first one:
  let args: Vec<String> = env::args().skip(1).collect();

  let pos = args.iter().position(|i| i == "-f")
    .expect("Please provide a config file with the -f option");

  let filename = args.get(pos + 1)
    .expect("Please provide a filename for the config file");

  // TODO We could match errors to give a more useful message here:
  let config = ConfigFile::from(filename)
    .expect("Error reading the config file - Make sure it exists and is readable");

  if config.get_certificates().is_empty() {
    panic!("The config file contained no certificate file paths to check");
  }

  run(args, config)

}
