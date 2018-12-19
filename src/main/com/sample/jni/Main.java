package com.sample.jni;

public class Main{

  static{
    System.loadLibrary("callbacks");
  }

  public void printInRust(){
    Library.printMsg();
  }
  public void returnInteger(){
    int i = Library.returnInt();
    System.out.println(i);
  }

  public void returnString(){
    String s = Library.returnString();
    System.out.println(s);
  }

  public void userDataTest(){
    UserData userData = UserData.createUser("Lionel", 100.0);
    System.out.println(userData.getUserInfo());
  }

  public void checkCallback(){
    UserData userData1 = UserData.createUser("Yogesh", 1345553.0);
    UserData.printUserData(userData1, (x) -> {
          System.out.println(x);
        });
  }

  public static void main(String[] args) {
    Main x = new Main();
    x.printInRust();
    x.returnInteger();
    x.returnString();
    x.userDataTest();
    x.checkCallback();
  }
}
