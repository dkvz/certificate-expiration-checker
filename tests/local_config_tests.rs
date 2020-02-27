#[path = "../src/local_config.rs"] mod local_config;
use local_config::*;

#[test]
fn get_an_error_when_file_does_not_exist() {
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
      assert_eq!(config.get_certificates().len(), 1);
      assert_eq!(config.get_certificates()[0], "snakeoil.pem");
    },
    Err(e) => panic!("Error reading fixture file: {}", e)
  }
}

#[test]
fn can_provide_empty_config_file() {
  match ConfigFile::from("tests/fixtures/empty.toml") {
    Ok(config) => {
      if config.get_notification_email().is_some() || 
        config.get_certificates().len() != 0 {
          panic!("Empty config has data");
        }
    },
    Err(e) => panic!("Error reading fixture file: {}", e)
  }
}