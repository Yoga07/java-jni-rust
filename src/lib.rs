extern crate jni;

use::jni::JNIEnv;

use jni::objects::{JClass,JString,JObject,JValue};

use jni::sys::jint;
#[no_mangle]

#[allow(non_snake_case)]

pub extern "system" fn Java_objectestprog_send(env :JNIEnv,
                                            _class: JClass,
                                            A:jint,
                                            B:jint,
                                            O:JObject){
    let p = JValue::Int(7);

    let q = JValue::from(A);

    let r = JValue::from(B);

    env.call_method(O,"print","(I)V",&[p]);

    env.call_method(O,"add","(II)V",&[q,r]);

}
