extern crate jni;

use ::jni::JNIEnv;
use jni::objects::{JClass, JObject, JString, JValue};
use jni::sys::{jint, jstring, jobject, jdouble};
use std::{thread, time};

#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn Java_Library_printMsg(
    _env: JNIEnv,
    _class: JClass
    ) {
    println!("Hello World!");
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_Library_returnInt(
    _env: JNIEnv,
    _class: JClass
    ) -> jint {
    let o: jint = 8;
    o
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_Library_returnString(
    _env: JNIEnv,
    _class: JClass
    ) -> jstring {
    let s = _env.new_string("hello").expect("error");
    s.into_inner()
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_UserData_createUser(
    _env: JNIEnv,
    _class: JClass,
    name: JString,
    balance: jdouble
    ) -> jobject {
    let userDataClass = _env
        .find_class("LUserData;")
        .expect("Could not find class");

    let _newUserData = _env
        .alloc_object(userDataClass)
        .expect("Could not allocate object");

    _env.set_field(
        _newUserData,
        "name",
        "Ljava/lang/String;",
        JValue::from(JObject::from(name))
    ).expect("Could not set name field");

    _env.set_field(
        _newUserData,
        "balance",
        "D",
        JValue::from(balance)
    ).expect("Could not set balance field");

    _newUserData.into_inner()
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_UserData_printUserData(
    _env: JNIEnv,
    _class: JClass,
    _obj1: JObject,
    obj2: JObject
    ) {
        let handle = thread::spawn(|| {
        println!("Rust is processing");
        let sleeptime = time::Duration::from_secs(3);
        thread::sleep(sleeptime);
        println!("Rust finishes processing");
        });
        let z = _env.new_string("Goes to Java").expect("error creating java string");
        let q = JValue::Object(JObject::from(z));
        _env.call_method(
            obj2,
            "call",
            "(Ljava/lang/String;)V",
            &[q]).expect("error in calling method from java");
        handle.join().unwrap();
}
