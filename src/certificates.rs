use std::io;
use std::io::Cursor;
use std::io::Read;
use std::fs::File;
use std::error::Error;
// A crate I use:
extern crate x509_parser;
use x509_parser::pem::{pem_to_der, Pem};
use x509_parser::parse_x509_der;

pub fn get_certificate_expiry_date(filename: &str) -> Result<&str, Box<Error>> {
  let bytes = read_bytes_from_file(filename)?;
  // Now decode the cert.
  // I need to map errors and recreate them I think.
  
  let res = pem_to_der(&bytes);
  let reader = Cursor::new(bytes);
  let (pem, bytes_read) = Pem::read(reader)?;
  let x509 = pem.parse_x509()?;
  // Uh... Find some string to return somewhere
  x509.tbs_certificate.issuer
}

fn read_bytes_from_file(filename: &str) -> Result<Vec<u8>, Box<Error>> {
  // I think it has to be mutable but I'm not certain.
  let mut f = File::open(filename)?;
  let mut buffer = Vec::new();
  f.read_to_end(&mut buffer)?;
  Ok(buffer)
}