#[path = "../src/local_config.rs"] mod local_config;
use local_config::*;
use std::net::{SocketAddr, Ipv4Addr, IpAddr};

#[test]
fn error_when_file_does_not_exist() {
  if let Ok(_) = ConfigFile::from("thisdoesnotexist.ini") {
    panic!("Creating a ConfigFile struct with a file 
      that does not exist did not result in an error");
  }
}

#[test]
fn can_read_fixture_email_1_cert() {
  match ConfigFile::from("tests/fixtures/example_email_1_cert.toml") {
    Ok(config) => {
      let email = config.get_notification_email().as_ref().unwrap();
      assert_eq!(email, "test@dkvz.eu");
      assert_eq!(config.get_from_email(), "certalert@my-machine.local");
      assert_eq!(config.get_certificates().len(), 1);
      assert_eq!(config.get_certificates()[0], "tests/fixtures/snakeoil.pem");
    },
    Err(e) => panic!("Error reading fixture file: {}", e)
  }
}

#[test]
fn can_provide_empty_config_file() {
  match ConfigFile::from("tests/fixtures/empty.toml") {
    Ok(config) => {
      if config.get_notification_email().is_some() || 
        !config.get_certificates().is_empty() {
          panic!("Empty config has data");
        }
    },
    Err(e) => panic!("Error reading fixture file: {}", e)
  }
}

#[test]
fn default_min_days_is_30() {
  match ConfigFile::from("tests/fixtures/empty.toml") {
    Ok(config) => {
      assert_eq!(*config.get_alert_min_days(), 30);
    },
    Err(e) => panic!("Error reading fixture file: {}", e)
  }
}

#[test]
fn negative_alert_min_days_is_inverted() {
  match ConfigFile::from("tests/fixtures/alert_min_days.toml") {
    Ok(config) => {
      assert_eq!(*config.get_alert_min_days(), 50);
    },
    Err(e) => panic!("Error reading fixture file: {}", e)
  }
}

#[test]
fn unparsable_alert_min_days_gets_default_value() {
  match ConfigFile::from("tests/fixtures/alert_min_days_float.toml") {
    Ok(config) => {
      assert_eq!(*config.get_alert_min_days(), 30);
    },
    Err(e) => panic!("Error reading fixture file: {}", e)
  }
}

#[test]
fn from_email_has_default_value() {
  match ConfigFile::from("tests/fixtures/empty.toml") {
    Ok(config) => {
      assert_eq!(config.get_from_email(), "nobody@localhost");
    },
    Err(e) => panic!("Error reading fixture file: {}", e)
  }
}

#[test]
fn smtp_host_has_default_value() {
  let default_smtp_host = SocketAddr::new(
    IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 
    25
  );
  match ConfigFile::from("tests/fixtures/empty.toml") {
    Ok(config) => {
      assert_eq!(config.get_smtp_host(), &default_smtp_host);
    },
    Err(e) => panic!("Error reading fixture file: {}", e)
  }
}