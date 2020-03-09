use std::error::Error;
use crate::{ProcessedCert};
extern crate lettre;
extern crate lettre_email;
use lettre_email::{Email, Mailbox};
use lettre::{Transport, SmtpClient};

pub fn send_email_notification(
  from: &String, dest: &String, certs: &Vec<ProcessedCert>
) -> Result<(), Box<dyn Error>> {
  let mut content = String::from("Some certificates approach expiration:\r\n---\r\n");
  let more_content = certs
    .iter()
    .map(|cert| format!("{}", cert))
    .collect::<Vec<String>>()
    .join("\r\n");
  content.push_str(&more_content);

  /*let email = SendableEmail::new(
    Envelope::new(
        Some(EmailAddress::new(from)?),
        vec![EmailAddress::new(dest)?],
    )?,
    "id".to_string(),
    content.into_bytes(),
  );*/
  let email = build_email(from, dest, &content)?;

  /*let mut mailer =
    SmtpClient::new_unencrypted_localhost()?.transport();
  // Send the email
  mailer.send(email.into())?;
  Ok(())*/

  send_email(email)
}

pub fn send_test_email(
  from: &String, dest: &String
) -> Result<(), Box<dyn Error>> {
  let email = build_email(
    from,
    dest,
    &String::from("Test email from the certificate expiration alert process")
  )?;
  send_email(email)
}

fn send_email(email: Email) -> Result<(), Box<dyn Error>> {
  let mut mailer =
      SmtpClient::new_unencrypted_localhost()?.transport();
  // Send the email
  mailer.send(email.into())?;
    
  Ok(())
}

fn build_email(
  from: &String, dest: &String, content: &String
) -> Result<Email, Box<dyn Error>> {
  // Error type is in lettre:Error:Error;
  let from = Mailbox::new(from.clone());
  let dest = Mailbox::new(dest.clone());
  let email = Email::builder()
    // Addresses can be specified by the tuple (email, alias)
    .to(dest)
    .from(from)
    .subject("Certificate expiration notice")
    .text(content)
    .build();

  // We could check email.is_ok().
  // Not going to do it here.
  Ok(email?)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn can_build_email() {
      match build_email(
        &String::from("valid@valid.com"), 
        &String::from("destination@valid.org"), 
        &String::from("Text content")
      ) {
        Err(error) => panic!("Error trying to build an email message: {}", error),
        _ => ()
      }
  }

  // TODO
  // This would be a good use case for parametrized tests.
  // If I knew how to do them that is.

  #[test]
  fn build_email_with_invalid_addresses_errors() {
    match build_email(
      &String::from("invalid"), 
      &String::from("pants"), 
      &String::from("Text content")
    ) {
      Ok(_) => panic!("Could build email with invalid email addresses"),
      _ => ()
    }
  }

}