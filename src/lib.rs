extern crate jni;

use ::jni::JNIEnv;
use jni::objects::{JClass, JObject, JString, JValue};
use jni::sys::{jint, jstring, jobject, jdouble, jclass};

#[no_mangle]
pub extern "system" fn Java_com_sample_jni_Library_printMsg(
    env: JNIEnv,
    _class: JClass
    ) {
    println!("Hello World!");
}

#[no_mangle]
pub extern "system" fn Java_com_sample_jni_Library_returnInt(
    env: JNIEnv,
    _class: JClass
    ) -> jint {
    let o: jint = 8;
    o
}

#[no_mangle]
pub extern "system" fn Java_com_sample_jni_Library_returnString(
    env: JNIEnv,
    _class: JClass
    ) -> jstring {
    let s = env.new_string("hello").expect("error");
    s.into_inner()
}

#[no_mangle]
pub extern "system" fn Java_com_sample_jni_UserData_createUser(
    env: JNIEnv,
    _class: JClass,
    name: JString,
    balance: jdouble
    ) -> jobject {
    let userDataClass = env
        .find_class("Lcom/sample/jni/UserData;")
        .expect("Could not find class");

    let newUserData = env
        .alloc_object(userDataClass)
        .expect("Could not allocate object");

    env.set_field(
        newUserData,
        "name",
        "Ljava/lang/String;",
        JValue::from(JObject::from(name))
    ).expect("Could not set name field");

    env.set_field(
        newUserData,
        "balance",
        "D",
        JValue::from(balance)
    ).expect("Could not set balance field");

    newUserData.into_inner()
}