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
And provide it to the executable using the "-f" flag.

The certificates have to hold a single public key. It will also work when multiple certificates are in a single file, but **only the first one will be checked for validity**.

Example:
**TODO**

## The general idea
Provide a config file with:
- A notification email address (optional)
- A list of certificates to check
  * For the moment: should be a single public key per file
- An optional amount of days before expiration that warrants notifying the user - Defaults to 30 days if absent or invalid

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
  * Using version 0.6 of x509_parser creates an infinite loop. Yeah.
* Sending emails through SMTP: https://github.com/lettre/lettre
* Colored terminal: https://github.com/mackwic/colored

I should use one for command line arguments but I figured I'd learn more things by doing it from scratch.

The validity date and time are in that struct: https://docs.rs/x509-parser/0.6.2/x509_parser/x509/struct.Validity.html

I'm using the "chrono" crate for dates and times parsing and creating from timestamps.

# TODO
- [ ] In main.rs, all the logic starting from `let max_ts` should be moved to lib.rs under a function named "run".
- [ ] I've been reading extern crate is no longer needed, is that true?
- [ ] Test the paths on Windows.
- [ ] Try reading something that we shouldn't parse, like a private key I could generate with OpenSSL.
- [ ] Am I doing things right by using that Box<Error> thing everywhere?
- [ ] Add tests for lib.rs.
- [ ] Add documentation in code - With "doc tests".
- [x] It would be cool to have colors in the final report.
- [ ] When panic is called, what is the return code from the program? Check with built executable too.
- [ ] Test if the latest version of the x509-parser crate passes the tests with no infinite loop now that they fixed the issue.
- [x] Add a "-q" flag to remove all output (exit code should still reflect the status though).
- [x] Check that panic! and expect print to stderr.
- [x] What happens if there's more than one cert in a file? -> Supposedly reads the first only.
- [x] Tell the crate authors about the infinite loop issue, I need to reproduce it in a test with a copy paste of the whole code.