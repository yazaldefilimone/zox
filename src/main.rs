mod bindings;
mod cli;
mod compiler;
mod core;
mod runtime;
mod utils;
// use compiler::transpiler;
use core::engine::Runtime;
// use utils::get_read_file;
fn main() {
  let args = cli::command_line(std::env::args().collect()).unwrap();
  let mut rt = Runtime::new();
  rt.eval_file(&args.file);
}
