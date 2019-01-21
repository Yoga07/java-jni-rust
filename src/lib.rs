extern crate jni;

use jni::objects::{JClass, JObject, JString, JValue};
use jni::sys::{jdouble, jint, jobject, jstring};
use jni::JNIEnv;
use std::{thread, time};

#[no_mangle]
#[allow(non_snake_case)]
#[allow(unused_variables)]
pub extern "system" fn Java_com_sample_jni_Library_printMsg(_env: JNIEnv, _class: JClass) {
    println!("[Rust]Hello World!");
}

#[no_mangle]
#[allow(non_snake_case)]
#[allow(unused_variables)]
pub extern "system" fn Java_com_sample_jni_Library_returnInt(_env: JNIEnv, _class: JClass) -> jint {
    let o: jint = 8;
    o
}

#[no_mangle]
#[allow(non_snake_case)]
#[allow(unused_variables)]
pub extern "system" fn Java_com_sample_jni_Library_returnString(
    _env: JNIEnv,
    _class: JClass,
) -> jstring {
    let s = _env.new_string("[Java]hello").expect("error");
    s.into_inner()
}

#[no_mangle]
#[allow(non_snake_case)]
#[allow(unused_variables)]
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
#[allow(non_snake_case)]
#[allow(unused_variables)]
pub extern "system" fn Java_com_sample_jni_UserData_printUserData(
    env: JNIEnv,
    _class: JClass,
    object_for_rust: JObject,
    sleep_value: jint,
    object_for_callbackAction: JObject,
) {
    let JVM = env.get_java_vm().unwrap();

    let RustObj = env.new_global_ref(object_for_rust).unwrap();

    let CallbackObj = env.new_global_ref(object_for_callbackAction).unwrap();



    //THREAD 1 does Rust functions
    let handle = thread::spawn(move || {
        let rustobj = RustObj.as_obj();
        println!("[Rust]Rust is processing");
        let sleep_time = time::Duration::from_secs(5);
        thread::park();
        thread::sleep(sleep_time);
        println!("[Rust]Rust finishes processing");
    });

    //THREAD 2 calls back to Java
    let handle1 = thread::spawn(move || {
        let env2 = JVM.attach_current_thread().unwrap();
        let callbackobj = CallbackObj.as_obj();
        println!("Rust->Calls back to Java");
        let z = env2
            .new_string("[Java]Java executes callbacks")
            .expect("error creating java string");
        let q = JValue::Object(JObject::from(z));
        env2.call_method(callbackobj, "call", "(Ljava/lang/String;)V", &[q])
            .expect("error in calling method from java");
        handle.thread().unpark();
        thread::sleep(time::Duration::from_secs(sleep_value as u64));
        println!("Java Responds");
    });
    //Giving time for the worker threads to be spawned
    thread::sleep(time::Duration::from_millis(10));
}
