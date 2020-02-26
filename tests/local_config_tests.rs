#[path = "../src/local_config.rs"] mod local_config;
use local_config::*;

#[test]
fn can_create_a_config_file() {
  let conf = local_config::ConfigFile::from("Test").unwrap();
  assert_eq!(conf.get_notification_email(), "test@test.org");
}