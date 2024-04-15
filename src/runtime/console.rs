use crate::bindings::print_value;
use rquickjs::{class::Trace, methods, Ctx, Value};
#[derive(Trace)]
#[rquickjs::class(rename_all = "camelCase")]
pub struct Console<'js> {
  pub ctx: Ctx<'js>,
}
#[methods]
impl<'js> Console<'js> {
  #[qjs(constructor)]
  pub fn new(ctx: Ctx<'js>) -> Self {
    Self { ctx }
  }

  //  todo: improve this code, create bindings for all console methods
  pub fn log(&self, _args: Value<'js>) {
    let raw = _args.as_raw();
    let js_context = self.ctx.as_raw().as_ptr();
    unsafe {
      print_value(js_context, raw);
    }
    // print this using the `console.log` method   in quickjs, but it is not working with  object and array

    // let value = <Value<'js> as FromJs>::from_js(&self.ctx, _args).unwrap();

    // let mut result = String::new();

    // if _args.is_array() {
    //   let mut _args = _args.as_array().unwrap();
    //   result.push_str("[ ");
    //   for i in 0.._args.len() {
    //     if i == _args.len() - 1 {
    //       result.push_str(&self.print_recursion(_args.get(i).unwrap()));
    //     } else {
    //       result.push_str(&self.print_recursion(_args.get(i).unwrap()));
    //       result.push_str(", ");
    //     }
    //   }
    //   result.push_str(" ]");
    // } else if _args.is_object() {
    //   result.push_str("{ ");
    //   let args = _args.as_object().unwrap().clone();
    //   args.into_iter().for_each(|_result| {
    //     if let Ok((key, value)) = _result {
    //       result.push_str(&format!(
    //         "{}: {}, ",
    //         key.to_string().unwrap().as_str(),
    //         self.print_recursion(value)
    //       ));
    //     }
    //   });
    //   result.push_str(" }");
    // } else {
    //   result.push_str(&self.print_recursion(_args));
    // }
    // println!("{}", result);
  }

  // pub fn print_recursion<'js>(&self, _args: Value<'js>) -> String {
  //   if _args.is_int() {
  //     format!("{}", _args.as_int().unwrap())
  //   } else if _args.is_string() {
  //     format!("{}", _args.as_string().unwrap().to_string().unwrap().as_str())
  //   } else if _args.is_array() {
  //     format!("[object Array]")
  //   } else if _args.is_function() {
  //     format!("[object Function]")
  //   } else if _args.is_symbol() {
  //     format!("[object Symbol]")
  //   } else if _args.is_bool() {
  //     format!("{}", _args.as_bool().unwrap())
  //   } else if _args.is_null() {
  //     format!("null")
  //   } else if _args.is_undefined() {
  //     format!("undefined")
  //   } else if _args.is_error() {
  //     format!("[object Error]")
  //   } else if _args.is_int() {
  //     format!("{}", _args.as_int().unwrap())
  //   } else if _args.is_object() {
  //     format!("[object Object]")
  //   } else {
  //     format!("unsupported value")
  //   }
  // }
}
