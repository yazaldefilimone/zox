use rquickjs::qjs;

pub unsafe extern "C" fn print_value(ctx: *mut qjs::JSContext, val: qjs::JSValue) {
  //  todo:  find a better way to print value like numbers, objects, arrays
  let str = qjs::JS_ToCString(ctx, val); // convert to string
  if !str.is_null() {
    //  check if the string is not null
    let c_str = std::ffi::CStr::from_ptr(str); // convert to c_str

    let string = c_str.to_str().unwrap(); // convert to string
    println!("{}", string); // print the string
    qjs::JS_FreeCString(ctx, str); // free the string
  }
}
