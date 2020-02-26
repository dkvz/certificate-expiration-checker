use std::error::Error;
extern crate config;

// We should create instances with a
// "from" method that can return a
// Result.

// Unless we put "pub" in front of 
// fields we need accessor functions.
pub struct ConfigFile<'a> {
  notification_email: String,
  certificates: Vec<&'a str>
}

impl <'a> ConfigFile<'a> {

  pub fn from(filename: &str) -> Result<ConfigFile, Box<dyn Error>> {
    Ok(ConfigFile {
      notification_email: String::from("test@test.org"),
      certificates: vec!["cert1", "cert2"]
    })
  }

  pub fn get_notification_email(&self) -> &str {
    &self.notification_email
  }

}
