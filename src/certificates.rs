use std::io;
use std::io::Cursor;
use std::io::Read;
use std::fs::File;
use std::fmt;
use std::fmt::{Display, Formatter};
// A crate I use:
extern crate x509_parser;
use x509_parser::pem::Pem;
use x509_parser::error::PEMError;

// Constants I use to find the private key sections
// in certificates:
type PrivateKeyDelimiter = (&'static[u8], &'static[u8]);
const PRIV_KEY_SEQUENCES: [PrivateKeyDelimiter; 2] = [
  (b"-----BEGIN PRIVATE KEY-----", b"-----END PRIVATE KEY-----"),
  (b"-----BEGIN RSA PRIVATE KEY-----", b"-----END RSA PRIVATE KEY-----")
];

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

impl Display for CertReadError {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    let desc = match self {
      CertReadError::IOError(err) => format!("Error: IO - {}", err),
      CertReadError::PEMError => String::from("Error: file is using the PEM format"),
      CertReadError::CertParseError => String::from("Error: could not parse certificate data")
    };
    write!(f, "{}", desc)
  }
}

pub fn get_certificate_expiry_date(filename: &str) -> Result<i64, CertReadError> {
  let mut bytes = read_bytes_from_file(filename)?;
  // First look for a possible PRIVATE KEY section as the
  // x509 parser won't work if that section is the first
  // in the file.
  // There's currently no way to put two if let on the same line.
  for (begin_seq, end_seq) in PRIV_KEY_SEQUENCES.iter() {
    if let Some(priv_key_start_pos) = find_subsequence(&bytes.as_slice(), begin_seq) {
      if let Some(priv_key_end_pos) = find_subsequence(&bytes.as_slice(), end_seq) {
        if priv_key_start_pos < priv_key_end_pos {
          bytes = [
            &bytes[0 .. priv_key_start_pos], 
            &bytes[priv_key_end_pos + end_seq.len() .. bytes.len()]
          ].concat();
          break;
        }
      }
    }
  }

  // Now decode the cert.
  // Do we use this "res" value?
  // -> No I don't think it does anything.
  //let res = pem_to_der(&bytes);
  let reader = Cursor::new(bytes);
  let (pem, _bytes_read) = Pem::read(reader)?;
  // To mix things up let's not use the From trait for this one:
  let x509 = pem.parse_x509().map_err(|_|  CertReadError::CertParseError)?;
  //Ok(format!("{}", x509.tbs_certificate.issuer))
  Ok(
    x509.tbs_certificate.validity.not_after.to_timespec().sec
  )
}

fn read_bytes_from_file(filename: &str) -> Result<Vec<u8>, io::Error> {
  // I think it has to be mutable but I'm not certain.
  let mut f = File::open(filename)?;
  let mut buffer = Vec::new();
  f.read_to_end(&mut buffer)?;
  Ok(buffer)
}

// Stole this from here: 
// https://stackoverflow.com/questions/35901547/how-can-i-find-a-subsequence-in-a-u8-slice
// The idea is to look for a private key part in the file and remove it from the byte array.
fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
  haystack.windows(needle.len()).position(|window| window == needle)
}