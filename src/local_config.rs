use std::error::Error;
extern crate config;
use config::*;

// We should create instances with a
// "from" method that can return a
// Result.

// Unless we put "pub" in front of 
// fields we need accessor functions.
pub struct ConfigFile {
  notification_email: Option<String>,
  certificates: Vec<String>
}

impl ConfigFile {

  pub fn from(filename: &str) -> Result<ConfigFile, Box<dyn Error>> {
    let mut conf = Config::default();
    conf.merge(File::with_name(filename))?;
    // From here we can use get_array and get_str from the
    // Config instance:
    let email : Option<String> = match conf.get_str("notification_email") {
      Ok(e) => Some(e),
      Err(_) => None
    };
    /*let certs : Vec<String> = match conf.get_array("certificates") {
      Ok(found_certs) => found_certs.iter()
        .map(|val| val.into_str()
        .unwrap_or(String::new()))
        .collect(),
      Err(_) => Vec::new()
    };*/
    // TODO I HAVE NO IDEA WHY into_iter WORKS BUT NOT iter
    let certs : Vec<String> = match conf.get_array("certificates") {
      Ok(found_certs) => found_certs.into_iter()
        .map(|val| val.into_str()
        .unwrap_or(String::new()))
        .collect::<Vec<String>>(),
      Err(_) => Vec::new()
    };

    Ok(ConfigFile {
      notification_email: email,
      certificates: certs
    })
  }

  pub fn get_notification_email(&self) -> &Option<String> {
    &self.notification_email
  }

  pub fn get_certificates(&self) -> &Vec<String> {
    &self.certificates
  }

}
