public class UserData {

    public String name;
    public double balance;

    public String getUserInfo() {
        return "[name]=" + name + ", [balance]=" + balance;
    }

    public static native UserData createUser(String name, double balance);

    public static native void printUserData(UserData user, CallbackAction action);

}
