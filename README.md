# Certificate Expiration Checker
I don't know how to Rust.

## How to use
**Work in progress**.

Prepare a toml config file such as:
```js
notification_email = "some_email@email.org"

certificates = [
  "/etc/ssl/certs/snakeoil.pem",
  "/etc/ssl/certs/another_one.pem"
]
```
The certificates have to hold a single public key

And provide it to the executable using the "-f" flag.

Example:
**TODO**

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
  * I'm not going to use OpenSSL, it either requries compiling the entire lib with the Rust program or linking to a system lib which I'd rather avoid.
* Parsing x509 in "pem" format: https://docs.rs/x509-parser/0.6.2/x509_parser/pem/index.html

I should use one for command line arguments but I figured I'd learn more things by doing it from scratch.

The validity date and time are in that struct: https://docs.rs/x509-parser/0.6.2/x509_parser/x509/struct.Validity.html

# TODO
- [ ] Test the paths on Windows
- [ ] What happens if there's more than one cert in a file?
- [ ] Am I doing things right by using that Box<Error> thing everywhere?
- [ ] I've seen something like #[cfg(test)] or something, what is that?