package com.sample.jni;

public class Library {
    static{
      System.loadLibrary("callbacks");
    }

    public static native void printMsg();

    public static native int returnInt();

    public static native String returnString();


}
