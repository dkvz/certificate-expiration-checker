use std::env;
mod local_config;
use local_config::ConfigFile;
mod certificates;
use certificates::*;
extern crate chrono;
use chrono::{TimeZone, Utc};

enum CertStatus {
  Error,
  Valid,
  Alert
}

fn format_timestamp(ts: i64) -> String {
  let dt = Utc.timestamp(ts, 0);
  dt.to_string()
}

fn display_result(path: &String, status: CertStatus, extra: &String) {
  let status_desc = match status {
    CertStatus::Error => "Error",
    CertStatus::Valid => "Valid",
    CertStatus::Alert => "Alert"
  };
  println!("\t{} - {} - {}", path, status_desc, extra);
}

fn process_cert(path: &String, max_ts: i64) {
  match get_certificate_expiry_date(path) {
    Ok(timestamp) => {
      let mut expires_at = String::from("cert expires on: ");
      expires_at.push_str(format_timestamp(timestamp).as_str());
      if timestamp < max_ts {
        // Will expire before the treshold.
        // I don't know how to spell threshhold.
        display_result(path, CertStatus::Alert, &expires_at);
      } else {
        display_result(path, CertStatus::Valid, &expires_at);
      }
    },
    Err(err) => {
      let extra = match err {
        CertReadError::IOError(io_err) => format!("IO error: {}", io_err),
        CertReadError::CertParseError => String::from("Certificate data could not be parsed - Is this a public key?"),
        CertReadError::PEMError => String::from("The file appears not to be a PEM certificate")
      };
      display_result(path, CertStatus::Error, &extra);
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

  // Get the timestamp the expiry date should be over to not proc
  // an alert:
  let max_ts = config.get_max_timestamp(Utc::now().timestamp());

  // Iterate and check each certificate.
  // The logic should probably be in a module.
  //println!("Certs found in the config file: {:?}", config.get_certificates());
  println!("Results:");
  config
    .get_certificates()
    .iter()
    .for_each(|path| process_cert(path, max_ts));

}
