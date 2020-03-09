use std::env;
/*mod local_config;
use local_config::ConfigFile;*/
use certexpchecker::run;
use std::process;

fn exit_error(msg: &str) {
  eprintln!("{}", msg);
  process::exit(1);
}

fn main() {
  // Collect command line args but skip the first one:
  let args: Vec<String> = env::args().skip(1).collect();

  match run(args) {
    Ok(exit_code) => process::exit(exit_code),
    Err(msg) => exit_error(&msg)  
  }

}
