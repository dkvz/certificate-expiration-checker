#[path = "../src/local_config.rs"] mod local_config;
use local_config::*;

#[test]
fn get_an_error_when_file_does_not_exist() {
  if let Ok(_) = ConfigFile::from("thisdoesnotexist.ini") {
    panic!("Creating a ConfigFile struct with a file 
      that does not exist did not result in an error");
  }
}