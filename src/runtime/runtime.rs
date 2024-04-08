use rand::Rng;
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

#[callback]
pub fn floor(ctx: JSContext, _function: JSObject, _this: JSObject, args: &[JSValue]) -> Result<JSValue, JSValue> {
  let value: f64 = args[0].to_number(&ctx).unwrap();
  let result = value.floor();
  Ok(JSValue::number(&ctx, result))
}

#[callback]
pub fn random(ctx: JSContext, _function: JSObject, _this: JSObject, _args: &[JSValue]) -> Result<JSValue, JSValue> {
  let result = rand::thread_rng().gen::<f64>();
  Ok(JSValue::number(&ctx, result))
}
