use rquickjs::Context;
use rquickjs::Runtime;

pub fn create_engine() -> (Runtime, Context) {
  let runtime = Runtime::new().unwrap();
  let context = Context::full(&runtime).unwrap();
  (runtime, context)
}
