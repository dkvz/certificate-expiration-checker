#[path = "../src/certificates.rs"] mod certificates;
use certificates::{ get_certificate_expiry_date };

#[test]
fn getting_some_data() {
  let issuer = get_certificate_expiry_date("snakeoil.pem")
    .expect("Got an error reading the certificate");
  println!("Issuer is: {}", issuer);
}


