use crate::bindings::{
  engine::{qjs::*, JS_DupValue_real, JS_FreeValue_real},
  utils::make_c_string,
  value::JsValue,
};
use core::fmt::{Debug, Formatter};

// unsafe impl Sync for JsRef {}
#[derive(PartialEq, Eq)]
pub struct JsRef {
  ctx: *mut JSContext,
  v: JSValue,
}

impl Debug for JsRef {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    unsafe {
      let ctx = self.ctx;
      let v = self.v;

      let ptr = JS_ToCStringLen2(ctx, std::ptr::null_mut(), v, 0);
      let s = if ptr.is_null() {
        String::new()
      } else {
        let cstr = std::ffi::CStr::from_ptr(ptr);
        let s = cstr.to_str().map(|s| s.to_string()).unwrap_or_default();
        JS_FreeCString(ctx, ptr);
        s
      };

      write!(f, "{}", s)
    }
  }
}

impl Clone for JsRef {
  fn clone(&self) -> Self {
    unsafe { Self { ctx: self.ctx, v: JS_DupValue_real(self.ctx, self.v) } }
  }
}

impl Drop for JsRef {
  fn drop(&mut self) {
    unsafe {
      let tag = JS_VALUE_GET_NORM_TAG_real(self.v);
      match tag {
        JS_TAG_JS_TAG_STRING
        | JS_TAG_JS_TAG_OBJECT
        | JS_TAG_JS_TAG_FUNCTION_BYTECODE
        | JS_TAG_JS_TAG_BIG_INT
        | JS_TAG_JS_TAG_BIG_FLOAT
        | JS_TAG_JS_TAG_BIG_DECIMAL
        | JS_TAG_JS_TAG_SYMBOL => JS_FreeValue_real(self.ctx, self.v),
        _ => {}
      }
    }
  }
}

unsafe impl Send for JsRef {}
pub trait AsObject {
  fn js_ref(&self) -> &JsRef;

  fn get(&self, key: &str) -> JsValue {
    unsafe {
      let js_ref = self.js_ref();
      let ctx = js_ref.ctx;
      let v = js_ref.v;
      let r = JS_GetPropertyStr(ctx, v, make_c_string(key).as_ptr().cast());
      JsValue::from_qjs_value(ctx, r)
    }
  }

  fn set(&mut self, key: &str, value: JsValue) -> JsValue {
    unsafe {
      let js_ref = self.js_ref();
      let ctx = js_ref.ctx;
      let this_obj = js_ref.v;
      let v = value.into_qjs_value();
      match JS_SetPropertyStr(ctx, this_obj, make_c_string(key).as_ptr().cast(), v) {
        1 => JsValue::Bool(true),
        0 => JsValue::Bool(false),
        _ => JsValue::Exception(JsException(JsRef { ctx, v: js_exception() })),
      }
    }
  }

  fn invoke(&mut self, fn_name: &str, argv: &[JsValue]) -> JsValue {
    unsafe {
      let js_ref = self.js_ref();
      let ctx = js_ref.ctx;
      let this_obj = js_ref.v;
      let mut argv: Vec<JSValue> = argv.iter().map(|v| v.get_qjs_value()).collect();
      let fn_name = JS_NewAtom(ctx, make_c_string(fn_name).as_ptr());
      let v = JS_Invoke(ctx, this_obj, fn_name, argv.len() as i32, argv.as_mut_ptr());
      JS_FreeAtom(ctx, fn_name);
      JsValue::from_qjs_value(ctx, v)
    }
  }

  fn delete(&mut self, key: &str) {
    unsafe {
      let js_ref = self.js_ref();
      let ctx = js_ref.ctx;
      let this_obj = js_ref.v;
      let prop_name = JS_NewAtom(ctx, make_c_string(key).as_ptr());
      JS_DeleteProperty(ctx, this_obj, prop_name, 0);
      JS_FreeAtom(ctx, prop_name);
    }
  }

  fn to_map(&self) -> Result<HashMap<String, JsValue>, JsException> {
    unsafe {
      let js_ref = self.js_ref();
      let ctx = js_ref.ctx;
      let obj = js_ref.v;

      let mut properties: *mut JSPropertyEnum = std::ptr::null_mut();
      let mut count: u32 = 0;

      let flags = (JS_GPN_STRING_MASK | JS_GPN_SYMBOL_MASK | JS_GPN_ENUM_ONLY) as i32;
      let ret = JS_GetOwnPropertyNames(ctx, &mut properties, &mut count, obj, flags);
      if ret != 0 {
        return Err(JsException(JsRef { ctx, v: js_exception() }));
      }

      let properties = DroppableValue::new(properties, |&mut properties| {
        for index in 0..count {
          let prop = properties.offset(index as isize);
          JS_FreeAtom(ctx, (*prop).atom);
        }
        js_free(ctx, properties as *mut std::ffi::c_void);
      });

      let mut map = HashMap::new();
      for index in 0..count {
        let prop = (*properties).offset(index as isize);
        let raw_value = JS_GetPropertyInternal(ctx, obj, (*prop).atom, obj, 0);
        let value = JsValue::from_qjs_value(ctx, raw_value);
        if let JsValue::Exception(e) = value {
          return Err(e);
        }

        let key_value = JsValue::from_qjs_value(ctx, JS_AtomToString(ctx, (*prop).atom));
        if let JsValue::Exception(e) = key_value {
          return Err(e);
        }
        if let JsValue::String(key_res) = key_value {
          let key = key_res.to_string();
          map.insert(key, value);
        }
      }
      Ok(map)
    }
  }

  fn to_string(&self) -> String {
    format!("{:?}", self.js_ref())
  }
}
