use rusty_jsc::JSValue;
use rusty_jsc_macros::callback;

#[callback]
pub fn log(ctx: JSContext, _function: JSObject, _this: JSObject, args: &[JSValue]) -> Result<JSValue, JSValue> {
  let message: Vec<String> = args
    .iter()
    .map(|arg| arg.to_string(&ctx).unwrap().to_string())
    .collect();

  println!("{}", message.join(" "));
  let result = JSValue::undefined(&ctx);
  Ok(result)
}
