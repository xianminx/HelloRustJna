extern crate jni;

use std::ffi::CString;
use std::os::raw::c_char;

use jni::JNIEnv;
use jni::objects::{JClass, JObject, JValue};


struct Point {
  x: i32,
  y: i32
}


pub type Callback = unsafe extern "C" fn(*const c_char) -> ();
pub type StructCallback = unsafe extern "C" fn(*const u8) -> ();



#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn invokeCallbackViaJNA(callback: Callback) {
    let s = CString::new("Hello from Rust").unwrap();
    unsafe { callback(s.as_ptr()); }
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn invokeCallbackViaJNA2(callback: StructCallback) {
    let point = Point{x: 10, y: 5};
    unsafe { callback(&point as *const u8); }
}


#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_example_hellorustjna_RustLibApi_invokeCallbackViaJNI(
    env: JNIEnv,
    _class: JClass,
    callback: JObject
) {
    let s = String::from("Hello from Rust");
    let response = env.new_string(&s)
        .expect("Couldn't create java string!");
    env.call_method(callback, "callback", "(Ljava/lang/String;)V",
                    &[JValue::from(JObject::from(response))]).unwrap();
}
