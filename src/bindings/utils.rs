pub fn make_c_string<T: Into<Vec<u8>>>(s: T) -> std::ffi::CString {
  std::ffi::CString::new(s).unwrap_or(Default::default())
}

unsafe fn to_u32(ctx: *mut JSContext, v: JSValue) -> Result<u32, String> {
  if JS_VALUE_GET_NORM_TAG_real(v) == JS_TAG_JS_TAG_INT {
    let mut r = 0u32;
    JS_ToUint32_real(ctx, &mut r as *mut u32, v);
    Ok(r)
  } else {
    Err("value is Not Int".into())
  }
}
