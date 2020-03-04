use std::fmt;
use std::fmt::{Display, Formatter};
extern crate chrono;
use chrono::{TimeZone, Utc};
mod certificates;
use certificates::*;

fn format_timestamp(ts: i64) -> String {
  let dt = Utc.timestamp(ts, 0);
  dt.to_string()
}

pub enum CertStatus {
  Error,
  Valid,
  Alert
}

pub struct ProcessedCert<'a> {
  path: &'a String,
  status: CertStatus,
  description: String
}

impl<'a> ProcessedCert<'a> {

  pub fn new(path: &'a String, max_ts: i64) -> Self {
    let mut desc: String;
    let status: CertStatus;
    match get_certificate_expiry_date(path) {
      Ok(timestamp) => {
        desc = String::from("cert expires on: ");
        desc.push_str(format_timestamp(timestamp).as_str());
        if timestamp < max_ts {
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

  pub fn is_alert_status(&self) -> bool {
    match self.status {
      CertStatus::Alert => true,
      _ => false
    }
  }
  
}

impl<'a> Display for ProcessedCert<'a> {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    let status_desc = match self.status {
      CertStatus::Error => "Error",
      CertStatus::Valid => "Valid",
      CertStatus::Alert => "Alert"
    };
    write!(f, "{} - {} - {}", self.path, status_desc, self.description)
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