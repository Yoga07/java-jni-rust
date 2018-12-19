extern crate jni;

use jni::JNIEnv;
use jni::objects::{JClass, JObject, JString, JValue};
use jni::sys::{jint, jstring, jobject, jdouble};
use std::{thread, time};

#[no_mangle]
pub extern "system" fn Java_com_sample_jni_Library_printMsg(
    _env: JNIEnv,
    _class: JClass
    ) {
    println!("[Rust]Hello World!");
}

#[no_mangle]
pub extern "system" fn Java_com_sample_jni_Library_returnInt(
    _env: JNIEnv,
    _class: JClass
    ) -> jint {
    let o: jint = 8;
    o
}

#[no_mangle]
pub extern "system" fn Java_com_sample_jni_Library_returnString(
    _env: JNIEnv,
    _class: JClass
    ) -> jstring {
    let s = _env.new_string("[Rust to Java]hello").expect("error");
    s.into_inner()
}

#[no_mangle]
pub extern "system" fn Java_com_sample_jni_UserData_createUser(
    env: JNIEnv,
    _class: JClass,
    name: JString,
    balance: jdouble
    ) -> jobject {
    let user_data_class = env
        .find_class("Lcom/sample/jni/UserData;")
        .expect("Could not find class");

    let new_user_data = env
        .alloc_object(user_data_class)
        .expect("Could not allocate object");

    env.set_field(
        new_user_data,
        "name",
        "Ljava/lang/String;",
        JValue::from(JObject::from(name)))
        .expect("Could not set name field");

    env.set_field(
        new_user_data,
        "balance",
        "D",
        JValue::from(balance))
        .expect("Could not set balance field");

    new_user_data.into_inner()
}

#[no_mangle]
pub extern "system" fn Java_com_sample_jni_UserData_printUserData(
    _env: JNIEnv,
    _class: JClass,
    _obj1: JObject,
    obj2: JObject
    ) {
        let handle = thread::spawn(|| {
        println!("[Rust]Rust is processing");
        let sleeptime = time::Duration::from_secs(3);
        thread::sleep(sleeptime);
        println!("[Rust]Rust finishes processing");
        });
        let z = _env.new_string("Rust->Calls back to Java\n[Java]Java executes callbacks").expect("error creating java string");
        let q = JValue::Object(JObject::from(z));
        _env.call_method(
            obj2,
            "call",
            "(Ljava/lang/String;)V",
            &[q])
            .expect("error in calling method from java");
        handle.join().unwrap();
}
