mod cli;
mod core;
mod runtime;
mod utils;
use core::set_property;
use core::{create_callback, get_context, get_property};
use runtime::runtime::floor;
use runtime::runtime::log;
use runtime::runtime::random;
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
  let log_callback = create_callback(&context, Some(log));
  // let floor_callback = create_callback(&context, Some(floor));
  // let random_callback = create_callback(&context, Some(random));

  let console = get_property(&global, &context, "console");
  // let math = get_property(&global, &context, "Math");

  // console.set_property(&context, "log", log_callback).unwrap();
  set_property(&console, &context, "log", log_callback).unwrap();
  // set_property(&math, &context, "floor", floor_callback).unwrap();
  // set_property(&math, &context, "random", random_callback).unwrap();
  // ---

  // math
  // let math = global.get_property(&context, "Math").to_object(&context).unwrap();
  // math
  //   .set_property(&context, "floor", JSValue::callback(&context, floor))
  //   .unwrap();
  // math
  //   .set_property(&context, "random", JSValue::callback(&context, random))
  //   .unwrap();
  let _ = context.evaluate_script(script.as_str(), STARTING_LINE_NUMBER);
}
