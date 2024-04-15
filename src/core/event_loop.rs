use crate::runtime::console;
use rquickjs::{CatchResultExt, Class};

use super::engine::create_engine;

pub fn engine(code: &str) {
  let (_runtime, context) = create_engine();
  let _ = context.with(|ctx| {
    // let global_ctx = ctx.globals();
    let console = Class::instance(ctx.clone(), console::Console { ctx: ctx.clone() }).unwrap();
    ctx.globals().set("console", console.clone()).unwrap();
    // eval
    ctx.eval::<(), _>(code).catch(&ctx).unwrap();
  });
}
