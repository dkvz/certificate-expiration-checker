use std::env;
use std::process;
mod local_config;
use local_config::ConfigFile;
extern crate chrono;
use chrono::Utc;
use certexpchecker::{ProcessedCert};
mod notifications;
use notifications::{send_email_notification, send_test_email};

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

  // Check if the "test email" flag is in the args. In which case
  // we send a test email and exit.
  if args.contains(&"-t".to_string()) {
    if let Some(dest_email) = config.get_notification_email() {
      println!("Sending test email...");
      match send_test_email(config.get_from_email(), dest_email) {
        Ok(_) => {
          println!("Test email sent successfully.");
          process::exit(0);
        },
        Err(error) => panic!("Error sending test email: {}", error)
      }
    } else {
      panic!("Missing destination_email in config file");
    }
  }

  // Get the timestamp the expiry date should be over to not proc
  // an alert:
  let max_ts = config.get_max_timestamp(Utc::now().timestamp());

  // Iterate and check each certificate.
  let processed_certs: Vec<ProcessedCert> = config
    .get_certificates()
    .iter()
    .map(|path| ProcessedCert::new(path, max_ts))
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
      match send_email_notification(
        config.get_from_email(), 
        dest_email, 
        &alert_certs
      ) {
        Err(error) => panic!("Error sending the notification email: {}", error),
        _ => ()
      }
    }
    process::exit(2);
  }

}
