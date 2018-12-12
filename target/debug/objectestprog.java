class objectestprog{

  static{
    System.loadLibrary("objectest");
  }

  public static native void send(int a,int b,realobj o);

  public static void main(String args[]){
    realobj x = new realobj();
    objectestprog.send(5,6,x);
  }
}


class realobj{
  int a,b,c;

  public void print(int a){
    System.out.println(a);
  }

  public void add(int x,int y){
    System.out.println(x+y);
  }
}
