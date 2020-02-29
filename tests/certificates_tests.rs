#[path = "../src/certificates.rs"] mod certificates;
use certificates::{ get_certificate_expiry_date };

#[test]
fn can_read_fixture_cert_validity() {
  let not_after: i64 = get_certificate_expiry_date("snakeoil.pem")
    .expect("Got an error reading the certificate");
  assert_eq!(1864816487, not_after);
}


