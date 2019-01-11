extern crate jni;

use jni::objects::{JClass, JObject, JString, JValue};
use jni::sys::{_jobject, jdouble, jint, jobject, jstring};
use jni::JNIEnv;
use jni::JavaVM;
use std::marker::Send;
use std::sync::mpsc::channel;
use std::{thread, time};


#[no_mangle]
pub extern "system" fn Java_com_sample_jni_Library_printMsg(_env: JNIEnv, _class: JClass) {
    println!("[Rust]Hello World!");
}

#[no_mangle]
pub extern "system" fn Java_com_sample_jni_Library_returnInt(_env: JNIEnv, _class: JClass) -> jint {
    let o: jint = 8;
    o
}

#[no_mangle]
pub extern "system" fn Java_com_sample_jni_Library_returnString(
    _env: JNIEnv,
    _class: JClass,
) -> jstring {
    let s = _env.new_string("[Java]hello").expect("error");
    s.into_inner()
}

#[no_mangle]
pub extern "system" fn Java_com_sample_jni_UserData_createUser(
    env: JNIEnv,
    _class: JClass,
    name: JString,
    balance: jdouble,
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
        JValue::from(JObject::from(name)),
    )
    .expect("Could not set name field");

    env.set_field(new_user_data, "balance", "D", JValue::from(balance))
        .expect("Could not set balance field");

    new_user_data.into_inner()
}

#[no_mangle]

pub extern "system" fn Java_com_sample_jni_UserData_printUserData(
    env: JNIEnv,
    _class: JClass,
    _obj1: JObject,
    obj2: JObject,
) {
    let JVM = env.get_java_vm().unwrap();

    let mut packet: Option<JObject> = Some(obj2);

    //THREAD 1 does Rust functions
    let handle = thread::spawn(move || {
        let env1 = JVM.attach_current_thread().unwrap();
        println!("[Rust]Rust is processing");
        let sleep_time = time::Duration::from_secs(10);
        thread::sleep(sleep_time);
        println!("[Rust]Rust finishes processing");
    });
    //THREAD 2 calls back to Java
    let handle2 = thread::spawn(move || {
        let env2 = JVM.attach_current_thread().unwrap();

        let y: &mut Option<JObject> = &mut packet;

        let mut object = y.take().unwrap();

        let z = env2
            .new_string("Rust->Calls back to Java\n[Java]Java executes callbacks")
            .expect("error creating java string");

        let q = JValue::Object(JObject::from(z));

        env2.call_method(object, "call", "(Ljava/lang/String;)V", &[q])
            .expect("error in calling method from java");
    });
}
