package com.sample.jni;

import org.junit.Test;

public class LibraryTest {

    static {
        System.loadLibrary("objectest");
    }

    @Test public void testSomeLibraryMethod() {
        Library.printMsg();
    }
}
