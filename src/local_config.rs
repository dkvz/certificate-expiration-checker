use std::error::Error;
extern crate config;
use config::*;

// We should create instances with a
// "from" method that can return a
// Result.

// Unless we put "pub" in front of
// fields we need accessor functions.
#[derive(Debug)]
pub struct ConfigFile {
    notification_email: Option<String>,
    from_email: String,
    certificates: Vec<String>,
    alert_min_days: u32,
}

impl ConfigFile {
    pub fn from(filename: &str) -> Result<ConfigFile, Box<dyn Error>> {
        let conf = Config::builder()
            .add_source(File::with_name(filename))
            .build()?;

        // From here we can use get_array and get_str from the
        // Config instance:
        let email: Option<String> = match conf.get_string("notification_email") {
            Ok(e) => Some(e),
            Err(_) => None,
        };

        let from_email = match conf.get_string("from_email") {
            Ok(m) => m,
            Err(_) => String::from("nobody@localhost"),
        };
        /*let certs : Vec<String> = match conf.get_array("certificates") {
          Ok(found_certs) => found_certs.iter()
            .map(|val| val.into_str()
            .unwrap_or(String::new()))
            .collect(),
          Err(_) => Vec::new()
        };*/
        // TODO I HAVE NO IDEA WHY into_iter WORKS BUT NOT iter
        // Which is probably because I have no idea what "into" does.
        let certs: Vec<String> = match conf.get_array("certificates") {
            Ok(found_certs) => found_certs
                .into_iter()
                .map(|val| val.into_string().unwrap_or(String::new()))
                .collect::<Vec<String>>(),
            Err(_) => Vec::new(),
        };

        let min_days: u32 = match conf.get_int("alert_min_days") {
            Ok(days) => {
                if days < 0 {
                    days.abs() as u32
                } else {
                    days as u32
                }
            }
            Err(_) => 30,
        };

        Ok(ConfigFile {
            notification_email: email,
            from_email: from_email,
            certificates: certs,
            alert_min_days: min_days,
        })
    }

    pub fn get_notification_email(&self) -> &Option<String> {
        &self.notification_email
    }

    pub fn get_from_email(&self) -> &String {
        &self.from_email
    }

    pub fn get_certificates(&self) -> &Vec<String> {
        &self.certificates
    }

    pub fn get_alert_min_days(&self) -> &u32 {
        &self.alert_min_days
    }

    pub fn get_max_timestamp(&self, now: i64) -> i64 {
        // Timestamps from the Time crate seem to be i64.
        now + ((self.alert_min_days as i64) * 86400)
    }
}
