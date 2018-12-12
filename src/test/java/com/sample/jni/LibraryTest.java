package com.sample.jni;


import org.junit.Assert;
import org.junit.Test;

public class LibraryTest {

    static {
        System.loadLibrary("objectest");
    }

    @Test
    public void printInRust() {
        Library.printMsg();
    }

    @Test
    public void returnInteger() {
        int i = Library.returnInt();
        Assert.assertEquals(8, i);
        System.out.println(Library.returnInt());
    }

    @Test
    public void returnString() {
        String s = Library.returnString();
        Assert.assertTrue(s.equals("hello"));
    }

    @Test
    public void userDataTest() {
        UserData userData = UserData.createUser("Lionel", 100.0);
        Assert.assertNotNull(userData);
        System.out.println(userData.getUserInfo());
    }
}