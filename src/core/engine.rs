use super::loader;
use quickjs_sys::{self, JS_NewRuntime};
use std::ffi::CString;
use std::fs::File;
use std::io::Read;

pub struct Runtime {
  runtime: *mut quickjs_sys::JSRuntime,
  context: *mut quickjs_sys::JSContext,
}

impl Drop for Runtime {
  fn drop(&mut self) {
    unsafe {
      quickjs_sys::JS_FreeRuntime(self.runtime);
    }
  }
}

pub struct Context {
  ctx: *mut JSContext,
}

impl Drop for Context {
  fn drop(&mut self) {
    unsafe {
      quickjs_sys::JS_FreeContext(self.ctx);
    }
  }
}

impl Context {
  pub fn new(rt: *mut JSRuntime) -> Self {
    unsafe {
      let ctx = quickjs_sys::JS_NewContext(rt);
      Self { ctx }
    }
  }
  pub fn get_ctx(&self) -> *mut JSContext {
    self.ctx
  }

  pub fn get_new_runtime(&self) -> *mut JSRuntime {
    unsafe { quickjs_sys::JS_GetRuntime(self.ctx) }
  }

  pub fn get_global_object(&self) -> *mut JSValue {
    unsafe { quickjs_sys::JS_GetGlobalObject(self.ctx) }
  }
}

impl Runtime {
  pub fn new() -> Self {
    unsafe {
      let raw_rt = JS_NewRuntime();
      let ctx = Context::new(raw_rt);
      quickjs_sys::JS_SetModuleLoaderFunc(raw_rt, None, Some(loader::module_loader), std::ptr::null_mut());
      Self { runtime: raw_rt, context: ctx.get_ctx() }
    }
  }

  pub fn get_ctx(&self) -> *mut JSContext {
    unsafe { quickjs_sys::JS_GetGlobalObject(self.context) }
  }
  pub fn eval(&self, code: &str) -> *mut JSValue {
    let c_code = CString::new(code).unwrap();
    unsafe {
      quickjs_sys::JS_Eval(
        self.context,
        c_code.as_ptr(),
        c_code.as_bytes().len() as i32,
        std::ptr::null(),
        0,
      )
    }
  }
  pub fn eval_file(&self, path: &str) -> *mut JSValue {
    let mut file = File::open(path).unwrap();
    let mut code = String::new();
    file.read_to_string(&mut code).expect("Failed to read file");
    self.eval(&code)
  }
  // usar quando for rodar um módulo
  pub fn eval_module(&self, path: &str) -> *mut JSValue {
    let mut file = File::open(path).unwrap();
    let mut code = String::new();
    file.read_to_string(&mut code).expect("Failed to read file");
    let c_code = CString::new(code).unwrap();
    unsafe {
      quickjs_sys::JS_Eval(
        self.context,
        c_code.as_ptr(),
        c_code.as_bytes().len() as i32,
        path.as_ptr() as *const i8,
        (quickjs_sys::JS_EVAL_TYPE_MODULE | quickjs_sys::JS_EVAL_FLAG_COMPILE_ONLY) as i32,
      )
    }
  }
  // usar quando for rodar um código que não precisa de retorno
  pub fn run(&self, code: &str) {
    let val = self.eval(code);
    unsafe {
      quickjs_sys::JS_FreeValue(self.context, val);
    }
  }
}
