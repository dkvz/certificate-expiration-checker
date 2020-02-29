#[path = "../src/certificates.rs"] mod certificates;
use certificates::{ get_certificate_expiry_date, CertReadError };

#[test]
fn can_read_fixture_cert_validity() {
  let not_after: i64 = get_certificate_expiry_date("snakeoil.pem")
    .expect("Got an error reading the certificate");
  assert_eq!(1864816487, not_after);
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