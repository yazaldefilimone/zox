use rusty_jsc::JSContext;

pub fn get_context() -> JSContext {
  JSContext::default()
}
