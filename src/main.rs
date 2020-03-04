use std::env;
use std::process;
mod local_config;
use local_config::ConfigFile;
extern crate chrono;
use chrono::Utc;
use certexpchecker::{ProcessedCert};

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

  // Get the timestamp the expiry date should be over to not proc
  // an alert:
  let max_ts = config.get_max_timestamp(Utc::now().timestamp());

  // Iterate and check each certificate.
  // The logic should probably be in a module.
  //println!("Certs found in the config file: {:?}", config.get_certificates());
  //println!("Results:");
  let processed_certs: Vec<ProcessedCert> = config
    .get_certificates()
    .iter()
    .map(|path| ProcessedCert::new(path, max_ts))
    .collect();

  if !args.contains(&"-q".to_string()) {
    println!("Results:");
    for cert in &processed_certs {
      println!("\t- {}", cert);
    }
  }
  
  let alert_certs: Vec<ProcessedCert> = processed_certs
    .into_iter()
    .filter(|cert| cert.is_alert_status())
    .collect();

  // If alert_certs is not empty, return exit code 2.
  // Check that panic returns 1 with the built executable.
  
  if !alert_certs.is_empty() {
    process::exit(2);
  }

}
