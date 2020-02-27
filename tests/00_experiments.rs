//use std::path::Path;

// To see any println output from tests we have
// to add the "--nocapture" option as in: 
// cargo test -- --nocapture

/*#[test]
fn path_stuff() {
  let path = Path::new("./");
  for entry in path.read_dir().expect("read_dir call failed") {
    if let Ok(entry) = entry {
        println!("{:?}", entry.path());
    }
  }
}*/