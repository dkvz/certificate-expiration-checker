use std::fmt;
use std::fmt::{Display, Formatter};
mod notifications;
use notifications::{send_email_notification, send_test_email};
mod local_config;
//use local_config::ConfigFile;
extern crate chrono;
use chrono::{TimeZone, Utc};
mod certificates;
use certificates::*;
extern crate colored;
use colored::*;

// I need the re-export or I have a strange type 
// mismatch in main.rs.
pub use local_config::ConfigFile;

fn format_timestamp(ts: i64) -> String {
  let dt = Utc.timestamp(ts, 0);
  dt.to_string()
}

pub enum CertStatus {
  Error,
  Valid,
  Alert,
  Expired
}

pub struct ProcessedCert<'a> {
  path: &'a String,
  status: CertStatus,
  description: String
}

impl<'a> ProcessedCert<'a> {

  pub fn new(path: &'a String, now: i64, max_ts: i64) -> Self {
    let mut desc: String;
    let status: CertStatus;
    match get_certificate_expiry_date(path) {
      Ok(timestamp) => {
        desc = String::from("cert expires on: ");
        desc.push_str(format_timestamp(timestamp).as_str());
        // max_ts is necessarily bigger than now.
        if timestamp < now {
          status = CertStatus::Expired;
        } else if timestamp < max_ts {
          // Will expire before the treshold.
          // I don't know how to spell threshhold.
          status = CertStatus::Alert;
        } else {
          status = CertStatus::Valid;
        }
      },
      Err(err) => {
        desc = match err {
          CertReadError::IOError(io_err) => format!("IO error: {}", io_err),
          CertReadError::CertParseError => String::from("Certificate data could not be parsed - Is this a public key?"),
          CertReadError::PEMError => String::from("The file appears not to be a PEM certificate")
        };
        status = CertStatus::Error;
      }
    }
    ProcessedCert {
      path: path,
      status: status,
      description: desc
    }
  }

  // Also send the errors / certs we could not read
  pub fn is_alert_status(&self) -> bool {
    match self.status {
      CertStatus::Alert | 
      CertStatus::Error | 
      CertStatus::Expired => true,
      _ => false
    }
  }

  pub fn status_description(&self) -> String {
    match self.status {
      CertStatus::Error => String::from("Error"),
      CertStatus::Valid => String::from("Valid"),
      CertStatus::Alert => String::from("Alert"),
      CertStatus::Expired => String::from("Expired")
    }
  }

  pub fn colored_status_description(&self) -> String {
    let status = self.status_description();
    match self.status {
      CertStatus::Error | CertStatus::Expired => status.red().to_string(),
      CertStatus::Valid => status.green().to_string(),
      CertStatus::Alert => status.yellow().to_string(),
    }
  }

  pub fn to_colored_string(&self) -> String {
    format!(
      "{} - {} - {}",
      self.path.bold(),
      self.colored_status_description(),
      self.description
    )
  }
  
}

impl<'a> Display for ProcessedCert<'a> {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(
      f, "{} - {} - {}", 
      self.path, 
      self.status_description(), 
      self.description
    )
  }
}

// At some point I had to remove every call to panic! because it's just
// not suitable to report errors from a CLI utility.
// My easiest getaway was using String as the error type here.
// I should have a look at that "failure" crate everybody is using for
// some reason.
pub fn run(args: Vec<String>) -> Result<i32, String> {
  let pos = match args.iter().position(|i| i == "-f") {
    Some(pos) => pos,
    None => return Err(String::from("Please provide a config file with the -f option"))
  };

  let filename = match args.get(pos + 1) {
    Some(filename) => filename,
    None => return Err(String::from("Please provide a filename for the config file"))
  };

  // TODO We could match errors to give a more useful message here:
  let config = match ConfigFile::from(filename) {
    Ok(config) => config,
    Err(_) => return Err(String::from("Error reading the config file - Make sure it exists and is readable"))
  };

  // Check if the "test email" flag is in the args. In which case
  // we send a test email and exit.
  if args.contains(&"-t".to_string()) {
    if let Some(dest_email) = config.get_notification_email() {
      println!("Sending test email...");
      match send_test_email(config.get_from_email(), dest_email) {
        Ok(_) => {
          println!("Test email sent successfully.");
          return Ok(0);
        },
        Err(error) => return Err(format!("Error sending test email: {}", error))
      }
    } else {
      return Err(format!("Missing destination_email in config file"));
    }
  }

  // The rest of the program requires certificates to be present
  // in the config:
  if config.get_certificates().is_empty() {
    return Err(String::from("The config file contained no certificate file paths to check"));
  }

  // Get the timestamp the expiry date should be over to not proc
  // an alert:
  let now = Utc::now().timestamp();
  let max_ts = config.get_max_timestamp(now);

  // Iterate and check each certificate.
  let processed_certs: Vec<ProcessedCert> = config
    .get_certificates()
    .iter()
    .map(|path| ProcessedCert::new(path, now, max_ts))
    .collect();

  if !args.contains(&"-q".to_string()) {
    println!("Results:");
    for cert in &processed_certs {
      println!("\t- {}", cert.to_colored_string());
    }
  }
  
  let alert_certs: Vec<ProcessedCert> = processed_certs
    .into_iter()
    .filter(|cert| cert.is_alert_status())
    .collect();

  // If alert_certs is not empty, return exit code 2.
  // Check that panic returns 1 with the built executable.
  
  if !alert_certs.is_empty() {
    // Check if we have a notification email set:
    if let Some(dest_email) = config.get_notification_email() {
      if let Err(error) = send_email_notification(
        config.get_from_email(), 
        dest_email, 
        &alert_certs
      ) {
        return Err(format!("Error sending the notification email: {}", error));
      }
    }
    // Always return status 2 if some certificates were in error or alert ot
    // expired state:
    Ok(2)
  } else {
    // All fine:
    Ok(0)
  }
}

/*
fn display_result(path: &String, status: CertStatus, extra: &String) {
  let status_desc = match status {
    CertStatus::Error => "Error",
    CertStatus::Valid => "Valid",
    CertStatus::Alert => "Alert"
  };
  println!("\t{} - {} - {}", path, status_desc, extra);
}

pub fn process_cert(path: &String, max_ts: i64) {
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
*/