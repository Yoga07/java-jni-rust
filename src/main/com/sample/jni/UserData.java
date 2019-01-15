package com.sample.jni;

public class UserData {

    public String name;
    public double balance;

    public String getUserInfo() {
        return "[Java]->Name=" + name + ", Balance=" + balance;
    }

    public static native UserData createUser(String name, double balance);

    public static native void printUserData(UserData user, int sleep, CallbackAction action);

}
