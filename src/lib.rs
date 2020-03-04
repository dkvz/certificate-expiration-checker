extern crate chrono;
use chrono::{TimeZone, Utc};
mod certificates;
use certificates::*;

pub enum CertStatus {
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
