pub mod constants;
pub mod primitives;
use std::io::Read;

pub fn get_read_file(path: &str) -> String {
  let mut file = std::fs::File::open(path).unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();
  contents
}
