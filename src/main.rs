mod cli;
mod core;
mod runtime;
mod utils;
use core::get_context;
use runtime::runtime::log;
use rusty_jsc::JSValue;
use utils::constants::STARTING_LINE_NUMBER;
use utils::get_read_file;

fn main() {
  let mut context = get_context();
  let args = cli::command_line(std::env::args().collect()).unwrap();
  let script = match args.command {
    cli::Command::Run => get_read_file(&args.file),
  };
  // runtime functions
  let global = context.get_global_object();
  let log_callback = JSValue::callback(&context, Some(log));

  let console = global.get_property(&context, "console").to_object(&context).unwrap();
  console.set_property(&context, "log", log_callback).unwrap();
  // ---
  let _ = context.evaluate_script(script.as_str(), STARTING_LINE_NUMBER);
}
