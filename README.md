# Certificate Expiration Checker
I don't know how to Rust.

## The general idea
Provide a config file with:
- A notification email address
- A list of certificates to check
  * For the moment: should be a single public key per file

Either run the executable in one shot (invoked through something like cron) or daemon mode. That's it.

## Structure
I think people write a lib.rs file to centralize everything to be used by main.rs. Gotta check actual projects to see how they distribute code into modules and files.