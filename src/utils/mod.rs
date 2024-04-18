pub mod constants;
pub mod primitives;
use relative_path::RelativePath;
use std::io::Read;

pub fn get_read_file(path: &str) -> String {
  let mut file = std::fs::File::open(path).unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();
  contents
}

pub fn resolve_simple(base: &str, name: &str) -> String {
  if name.starts_with('.') {
    let path = RelativePath::new(base);
    if let Some(dir) = path.parent() {
      return dir.join_normalized(name).to_string();
    }
  }
  name.into()
}

pub fn check_extensions(name: &str, extensions: &[String]) -> bool {
  let path = RelativePath::new(name);
  path
    .extension()
    .map(|extension| extensions.iter().any(|known_extension| known_extension == extension))
    .unwrap_or(false)
}
