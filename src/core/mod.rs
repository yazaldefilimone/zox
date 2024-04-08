use rusty_jsc::{JSContext, JSObject, JSObjectCallAsFunctionCallback, JSValue};

pub fn get_context() -> JSContext {
  JSContext::default()
}

pub fn get_property(object: &JSObject, ctx: &JSContext, name: &str) -> JSObject {
  object.get_property(&ctx, "console").to_object(&ctx).unwrap()
}

pub fn create_callback(ctx: &JSContext, function: JSObjectCallAsFunctionCallback) -> JSValue {
  JSValue::callback(ctx, function)
}

pub fn set_property(object: &JSObject, ctx: &JSContext, name: &str, value: JSValue) -> Result<(), JSValue> {
  object.set_property(&ctx, name, value)
}
