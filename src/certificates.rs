use std::io;
use std::io::Cursor;
use std::io::Read;
use std::fs::File;
// A crate I use:
extern crate x509_parser;
use x509_parser::pem::{pem_to_der, Pem};
use x509_parser::error::PEMError;

#[derive(Debug)]
pub enum CertReadError {
  IOError(io::Error),
  PEMError,
  CertParseError
}

impl From<io::Error> for CertReadError {
  fn from(err: io::Error) -> Self {
    CertReadError::IOError(err)
  }
}

impl From<PEMError> for CertReadError {
  fn from(_: PEMError) -> Self {
    CertReadError::PEMError
  }
}

pub fn get_certificate_expiry_date(filename: &str) -> Result<String, CertReadError> {
  let bytes = read_bytes_from_file(filename)?;
  // Now decode the cert.
  // TODO Do we use this "res" value?
  let res = pem_to_der(&bytes);
  let reader = Cursor::new(bytes);
  let (pem, _bytes_read) = Pem::read(reader)?;
  // To mix things up let's not use the From trait for this one:
  let x509 = pem.parse_x509().map_err(|_|  CertReadError::CertParseError)?;
  // Uh... Find some string to return somewhere
  Ok(format!("{}", x509.tbs_certificate.issuer))
}

fn read_bytes_from_file(filename: &str) -> Result<Vec<u8>, io::Error> {
  // I think it has to be mutable but I'm not certain.
  let mut f = File::open(filename)?;
  let mut buffer = Vec::new();
  f.read_to_end(&mut buffer)?;
  Ok(buffer)
}