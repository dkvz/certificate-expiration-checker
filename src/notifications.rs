use std::error::Error;
use certexpchecker::{ProcessedCert};
extern crate lettre;
extern crate lettre_email;
use lettre_email::{Email};
use lettre::{SendableEmail, EmailAddress, Transport, Envelope, SmtpClient};


// Let's implement the custom aggregate error as a struct this time.
// TODO Check how they do this in other projects.



// TODO Add tests for the private method to build email directly
// inside of this file with the cfg tests thingy.


pub fn send_email_notification(from: &String, dest: &String, certs: &Vec<ProcessedCert>) 
  -> Result<(), Box<dyn Error>> {
    // Can we use fold to create the body from the certs vector?
    let mut content = String::from("Some certificates approach expiration:\r\n");
    let more_content = certs
      .iter()
      .map(|cert| format!("{}", cert))
      .collect()
      .join("\r\n");
    content.push_str(more_content);

    /*let email = SendableEmail::new(
      Envelope::new(
          Some(EmailAddress::new(from)?),
          vec![EmailAddress::new(dest)?],
      )?,
      "id".to_string(),
      content.into_bytes(),
    );*/
    let email = build_email(from, dest, content)?;

    let mut mailer =
      SmtpClient::new_unencrypted_localhost()?.transport();
    // Send the email
    mailer.send(email)?;
    
    Ok(())
  }

fn build_email(from: &String, dest: &String, content: &String) 
  -> Result<Email, Box<dyn Error>> {
    // TODO Check what errors these are returning:
    let from = EmailAddress::new(from)?;
    let dest = EmailAddress::new(dest)?;
    let email = Email::builder()
      // Addresses can be specified by the tuple (email, alias)
      .to(dest)
      .from(from)
      .subject("Certificate expiration notice")
      .text(content)
      .build();

      // TODO We should check email.is_ok()
      email
  }