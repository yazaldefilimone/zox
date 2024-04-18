use crate::bindings::*;
pub enum JsValue {
  Int(i32),
  Float(f64),
  BigNum(JsBigNum),
  String(JsString),
  Module(JsModule),
  Object(JsObject),
  Array(JsArray),
  Promise(JsPromise),
  ArrayBuffer(JsArrayBuffer),
  Function(JsFunction),
  Symbol(JsRef),
  Bool(bool),
  Null,
  UnDefined,
  Exception(JsException),
  FunctionByteCode(JsFunctionByteCode),
  Other(JsRef),
}

impl JsValue {
  pub fn from_qjs_value(ctx: *mut JSContext, v: JSValue) -> Self {
    unsafe {
      let tag = JS_VALUE_GET_NORM_TAG_real(v);
      match tag {
        JS_TAG_JS_TAG_INT => {
          let mut num = 0;
          JS_ToInt32(ctx, (&mut num) as *mut i32, v);
          JsValue::Int(num)
        }
        JS_TAG_JS_TAG_FLOAT64 => {
          let mut num = 0_f64;
          JS_ToFloat64(ctx, (&mut num) as *mut f64, v);
          JsValue::Float(num)
        }
        JS_TAG_JS_TAG_BIG_DECIMAL | JS_TAG_JS_TAG_BIG_INT | JS_TAG_JS_TAG_BIG_FLOAT => {
          JsValue::BigNum(JsBigNum(JsRef { ctx, v }))
        }
        JS_TAG_JS_TAG_STRING => JsValue::String(JsString(JsRef { ctx, v })),
        JS_TAG_JS_TAG_MODULE => JsValue::Module(JsModule(JsRef { ctx, v })),
        JS_TAG_JS_TAG_OBJECT => {
          if JS_IsFunction(ctx, v) != 0 {
            JsValue::Function(JsFunction(JsRef { ctx, v }))
          } else if JS_IsArrayBuffer(ctx, v) != 0 {
            JsValue::ArrayBuffer(JsArrayBuffer(JsRef { ctx, v }))
          } else if JS_IsArray(ctx, v) != 0 {
            JsValue::Array(JsArray(JsRef { ctx, v }))
          } else if JS_IsPromise(ctx, v) != 0 {
            JsValue::Promise(JsPromise(JsRef { ctx, v }))
          } else {
            JsValue::Object(JsObject(JsRef { ctx, v }))
          }
        }
        JS_TAG_JS_TAG_BOOL => JsValue::Bool(JS_ToBool(ctx, v) != 0),
        JS_TAG_JS_TAG_NULL => JsValue::Null,
        JS_TAG_JS_TAG_EXCEPTION => JsValue::Exception(JsException(JsRef { ctx, v })),
        JS_TAG_JS_TAG_UNDEFINED => JsValue::UnDefined,
        JS_TAG_JS_TAG_FUNCTION_BYTECODE => JsValue::FunctionByteCode(JsFunctionByteCode(JsRef { ctx, v })),
        JS_TAG_JS_TAG_SYMBOL => JsValue::Symbol(JsRef { ctx, v }),
        _ => JsValue::Other(JsRef { ctx, v }),
      }
    }
  }

  fn get_qjs_value(&self) -> JSValue {
    unsafe {
      match self {
        // JS_NewInt32 dont need ctx
        JsValue::Int(v) => JS_NewInt32_real(std::ptr::null_mut(), *v),
        // JS_NewFloat64 dont need ctx
        JsValue::Float(v) => JS_NewFloat64_real(std::ptr::null_mut(), *v),
        JsValue::BigNum(JsBigNum(JsRef { v, .. })) => *v,
        JsValue::String(JsString(JsRef { v, .. })) => *v,
        JsValue::Module(JsModule(JsRef { v, .. })) => *v,
        JsValue::Object(JsObject(JsRef { v, .. })) => *v,
        JsValue::Array(JsArray(JsRef { v, .. })) => *v,
        JsValue::ArrayBuffer(JsArrayBuffer(JsRef { v, .. })) => *v,
        JsValue::Function(JsFunction(JsRef { v, .. })) => *v,
        JsValue::Promise(JsPromise(JsRef { v, .. })) => *v,
        JsValue::Bool(b) => JS_NewBool_real(std::ptr::null_mut(), if *b { 1 } else { 0 }),
        JsValue::Null => js_null(),
        JsValue::UnDefined => js_undefined(),
        JsValue::Exception(JsException(JsRef { v, .. })) => *v,
        JsValue::FunctionByteCode(JsFunctionByteCode(JsRef { v, .. })) => *v,
        JsValue::Other(JsRef { v, .. }) => *v,
        JsValue::Symbol(JsRef { v, .. }) => *v,
      }
    }
  }

  fn into_qjs_value(self) -> JSValue {
    let s = std::mem::ManuallyDrop::new(self);
    s.get_qjs_value()
  }
}

impl JsValue {
  pub fn get(&self, key: &str) -> Option<JsValue> {
    match &self {
      JsValue::Object(obj) => Some(obj.get(key)),
      JsValue::Function(obj) => Some(obj.get(key)),
      JsValue::Array(obj) => Some(obj.get(key)),
      _ => None,
    }
  }
  pub fn index(&self, index: usize) -> Option<JsValue> {
    if let JsValue::Array(arr) = self {
      Some(arr.take(index))
    } else {
      None
    }
  }
  pub fn is_exception(&self) -> bool {
    if let JsValue::Exception(_) = self {
      true
    } else {
      false
    }
  }
  pub fn invoke(&mut self, fn_name: &str, argv: &[JsValue]) -> Option<JsValue> {
    if let JsValue::Object(obj) = self {
      Some(obj.invoke(fn_name, argv))
    } else {
      None
    }
  }
  pub fn to_obj(self) -> Option<JsObject> {
    if let JsValue::Object(o) = self {
      Some(o)
    } else {
      None
    }
  }
  pub fn to_function(self) -> Option<JsFunction> {
    if let JsValue::Function(o) = self {
      Some(o)
    } else {
      None
    }
  }
  pub fn to_array(self) -> Option<JsArray> {
    if let JsValue::Array(o) = self {
      Some(o)
    } else {
      None
    }
  }
  pub fn to_string(self) -> Option<JsString> {
    if let JsValue::String(s) = self {
      Some(s)
    } else {
      None
    }
  }
}

impl From<i32> for JsValue {
  fn from(v: i32) -> Self {
    Self::Int(v)
  }
}

impl From<f64> for JsValue {
  fn from(v: f64) -> Self {
    Self::Float(v)
  }
}

impl From<JsBigNum> for JsValue {
  fn from(v: JsBigNum) -> Self {
    Self::BigNum(v)
  }
}

impl From<JsString> for JsValue {
  fn from(v: JsString) -> Self {
    Self::String(v)
  }
}

impl From<JsModule> for JsValue {
  fn from(v: JsModule) -> Self {
    Self::Module(v)
  }
}

impl From<JsObject> for JsValue {
  fn from(v: JsObject) -> Self {
    Self::Object(v)
  }
}

impl From<JsArray> for JsValue {
  fn from(v: JsArray) -> Self {
    Self::Array(v)
  }
}

impl From<JsPromise> for JsValue {
  fn from(v: JsPromise) -> Self {
    Self::Promise(v)
  }
}

impl From<JsArrayBuffer> for JsValue {
  fn from(v: JsArrayBuffer) -> Self {
    Self::ArrayBuffer(v)
  }
}

impl From<JsFunction> for JsValue {
  fn from(v: JsFunction) -> Self {
    Self::Function(v)
  }
}

impl From<bool> for JsValue {
  fn from(v: bool) -> Self {
    Self::Bool(v)
  }
}

impl From<JsException> for JsValue {
  fn from(v: JsException) -> Self {
    Self::Exception(v)
  }
}

impl From<JsFunctionByteCode> for JsValue {
  fn from(v: JsFunctionByteCode) -> Self {
    Self::FunctionByteCode(v)
  }
}

impl From<JsRef> for JsValue {
  fn from(v: JsRef) -> Self {
    Self::from_qjs_value(v.ctx, v.v)
  }
}

impl From<()> for JsValue {
  fn from(_: ()) -> Self {
    JsValue::Null
  }
}
