# Certificate Expiration Checker
I don't know how to Rust.

## The general idea
Provide a config file with:
- A notification email address
- A list of certificates to check
  * For the moment: should be a single public key per file

Config file can be given using the "-f" argument, or the program looks for a default config file in current directory.

Either run the executable in one shot (invoked through something like cron) or daemon mode. That's it.

## Structure
I think people write a lib.rs file to centralize everything to be used by main.rs. Gotta check actual projects to see how they distribute code into modules and files.

## Crates and stuff
* Using config files: https://docs.rs/config/0.10.1/config/
  * Simple practical example here: https://github.com/mehcode/config-rs/blob/master/examples/simple/src/main.rs
* Parsing x509 certs using OpenSSL: https://docs.rs/openssl/0.9.14/openssl/x509/struct.X509.html

I should use one for command line arguments but I figured I'd learn more things by doing it from scratch.