use std::env;
mod local_config;
use local_config::ConfigFile;
mod certificates;
use certificates::*;

fn process_cert(path: &String) {
  match get_certificate_expiry_date(path) {
    Ok(timestamp) => println!("Timestamp: {}", timestamp),
    Err(err) => {
      println!("Some error happened: {}", err);
    }
  }
}

fn main() {
  let args: Vec<String> = env::args().collect();

  let pos = args.iter().position(|i| i == "-f")
    .expect("Please provide a config file with the -f option");

  let filename = args.get(pos + 1)
    .expect("Please provide a filename for the config file");

  // We could match errors to give a more useful message here:
  let config = ConfigFile::from(filename)
    .expect("Error reading the config file - Make sure it exists and is readable");

  if config.get_certificates().is_empty() {
    panic!("The config file contained no certificate file paths to check");
  }

  // Iterate and check each certificate.
  // The logic should probably be in a module.
  //println!("Certs found in the config file: {:?}", config.get_certificates());
  config.get_certificates()
    .iter().for_each(process_cert);

}
