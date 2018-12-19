import java.util.Objects;

import java.util.concurrent.CompletableFuture;

public class Library {

    static{
      System.loadLibrary("callbacks");
    }

    public static native void printMsg();

    public static native int returnInt();

    public static native String returnString();

    public static void main(String[] args) {

      Library.printMsg();

      System.out.println(Library.returnInt());

      String s = Library.returnString();
      System.out.println(s);

      UserData userData = UserData.createUser("Lionel", 100.0);
      System.out.println(userData.getUserInfo());

      UserData userData1 = UserData.createUser("Yogesh", 1345553.0);
      UserData.printUserData(userData1, (x) -> {
            System.out.println(x);
        });
    }
}
