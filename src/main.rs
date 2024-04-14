mod cli;
mod core;
mod runtime;
mod utils;
use core::engine;
use rquickjs::{CatchResultExt, Class};
use runtime::console;
use utils::get_read_file;

fn main() {
  let args = cli::command_line(std::env::args().collect()).unwrap();
  let script_code = match args.command {
    cli::Command::Run => get_read_file(&args.file),
  };
  let _absolute_path = args.file;

  let (_runtime, context) = engine::create_engine();
  let _ = context.with(|ctx| {
    // let global_ctx = ctx.globals();
    let console = Class::instance(ctx.clone(), console::Console {}).unwrap();
    ctx.globals().set("console", console.clone()).unwrap();
    ctx.eval::<(), _>(script_code).catch(&ctx).unwrap();
  });
}
