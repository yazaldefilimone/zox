mod bindings;
mod cli;
mod compiler;
mod core;
mod runtime;
mod utils;
use compiler::transpiler;
use core::event_loop;
use utils::get_read_file;

fn main() {
  let args = cli::command_line(std::env::args().collect()).unwrap();
  let script_code = match args.command {
    cli::Command::Run => get_read_file(&args.file),
    cli::Command::Test => panic!("Test command not implemented"),
  };
  let _absolute_path = args.file;
  let code = match args.extension {
    cli::FileType::TypeScript => transpiler::transform(&_absolute_path, &script_code),
    cli::FileType::JavaScript => script_code,
    _ => panic!("Invalid file extension"),
  };
  event_loop::engine(&code);
}
