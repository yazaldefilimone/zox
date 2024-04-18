unsafe extern "C" fn module_loader(
  ctx: *mut JSContext,
  module_name_: *const ::std::os::raw::c_char,
  _opaque: *mut ::std::os::raw::c_void,
) -> *mut JSModuleDef {
  let module_name = std::ffi::CStr::from_ptr(module_name_).to_str();
  if module_name.is_err() {
    return std::ptr::null_mut();
  }
  let module_name = module_name.unwrap();

  let mut path = std::path::PathBuf::from(module_name);
  let ext = path.extension().unwrap_or_default().to_str().unwrap_or_default();
  match ext {
    "" => {
      path.set_extension("js");
    }
    "js" => {}
    _ => {
      JS_ThrowReferenceError(
        ctx,
        "could not load module filename '%s'\0".as_ptr().cast(),
        module_name_,
      );
      return std::ptr::null_mut();
    }
  }

  if !path.is_file() {
    let modules_dir = std::env::var("QJS_LIB").unwrap_or("./modules".to_string());
    path = std::path::PathBuf::from(modules_dir).join(path);
  }

  let code = std::fs::read(&path);
  if code.is_err() {
    JS_ThrowReferenceError(
      ctx,
      "could not load module filename '%s'\0".as_ptr().cast(),
      module_name_,
    );
    return std::ptr::null_mut();
  }

  let buf = code.unwrap();
  let buf_len = buf.len();
  let buf = make_c_string(buf);

  // compile the module
  let func_val = JS_Eval(
    ctx,
    buf.as_ptr(),
    buf_len,
    module_name_,
    (JS_EVAL_TYPE_MODULE | JS_EVAL_FLAG_COMPILE_ONLY) as i32,
  );

  if JS_IsException_real(func_val) != 0 {
    return std::ptr::null_mut();
  }

  js_module_set_import_meta(ctx, func_val, 0, 0);

  let m = JS_VALUE_GET_PTR_real(func_val);
  JS_FreeValue_real(ctx, func_val);

  m.cast()
}
