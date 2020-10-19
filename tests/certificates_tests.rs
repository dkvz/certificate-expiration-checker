#[path = "../src/certificates.rs"] mod certificates;
use certificates::{ get_certificate_expiry_date, CertReadError };

// Testing with a basic self signed cert from some Linux VM.
#[test]
fn can_read_fixture_cert_validity1() {
  let not_after: i64 = get_certificate_expiry_date("tests/fixtures/snakeoil.pem")
    .expect("Got an error reading the certificate");
  assert_eq!(1864816487, not_after);
}

// Testing with an intermediate cert from RapidSSL.
#[test]
fn can_read_fixture_cert_validity2() {
  let not_after: i64 = get_certificate_expiry_date("tests/fixtures/rapidssl_intermediate.crt")
    .expect("Got an error reading the certificate");
  assert_eq!(1825503813, not_after);
}

#[test]
fn error_when_file_does_not_exist() {
  if let Err(read_error) = get_certificate_expiry_date("thisdoesnotexist.pem") {
    match read_error {
      CertReadError::IOError(_) => (),
      _ => panic!("Non existent file caused an error that is **not** an IOError")
    }
  } else {
    panic!("Non existent file passed through get_certificate_expiry_date with no error");
  }
}

#[test]
fn error_when_file_is_not_pem_format() {
  if let Err(cert_error) = get_certificate_expiry_date("tests/fixtures/example_email_1_cert.toml") {
    match cert_error {
      CertReadError::PEMError => (),
      _ => panic!("Parsing a file that isn't PEM-formatted return an error that is not CertReadError::PEMError")
    }
  } else {
    panic!("Could parse a non-certificate file with no error");
  }
}

#[test]
fn works_when_file_has_multiple_certs() {
  let not_after: i64 = get_certificate_expiry_date("tests/fixtures/multiple_certs.crt")
    .expect("Could not parse the certificate");
  assert_eq!(1864816487, not_after);
}

#[test]
fn can_read_fixture_starting_with_private_key() {
  let not_after: i64 = get_certificate_expiry_date("tests/fixtures/ex_cert_full.pem")
    .expect("Got an error reading the certificate");
  assert_eq!(1603276162, not_after);
}

#[test]
fn error_when_file_only_contains_private_key() {
  if let Err(cert_error) = get_certificate_expiry_date("tests/fixtures/ex_cert_key.pem") {
    match cert_error {
      CertReadError::PEMError => (),
      _ => panic!("Parsing just a private key returned an error that is not CertReadError::PEMError")
    }
  } else {
    panic!("Parsing just a private key yielded no error whereas it should have");
  }
}

#[test]
fn can_read_fixture_ending_with_private_key() {
  // Not sure this is a valid format (with private key at the end),
  // but we can test it anyway - It's important that it doesn't make
  // the program panic for other reasons too.
  let not_after: i64 = get_certificate_expiry_date("tests/fixtures/ex_cert_full_inverted.pem")
    .expect("Got an error reading the certificate");
  assert_eq!(1603276162, not_after);
}

#[test]
fn can_read_fixture_with_private_key_in_the_middle() {
  // Not sure this is a valid format (with private key at the end),
  // but we can test it anyway - It's important that it doesn't make
  // the program panic for other reasons too.
  let not_after: i64 = get_certificate_expiry_date("tests/fixtures/private_key_in_the_middle.pem")
    .expect("Got an error reading the certificate");
  assert_eq!(1603276162, not_after);
}

#[test]
fn error_no_private_key_end_tag() {
  if let Err(cert_error) = get_certificate_expiry_date("tests/fixtures/private_key_no_endtag.pem") {
    match cert_error {
      CertReadError::PEMError => (),
      _ => panic!("Invalid cert produced an error that isn't CertReadError::PEMError")
    }
  } else {
    panic!("Parsing invalid cert produced no error");
  }
}