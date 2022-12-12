public class tests {
    public static void main(String[] args) {
        FavNums f = new FavNums(0, 0, 0);
        f.printRandoms();
    }
}

class FavNums {
    private int num1;
    private int num2;
    private int num3;
  
    public FavNums(int n1, int n2, int n3) {
      num1 = n1;
      num2 = n2;
      num3 = n3;
    }
  
    public void printFavorites() {
      System.out.println(num1 + num2 + num3);
    }
  
    public static void printRandoms() {
      int r1 = (int)(100 * Math.random());
      int r2 = (int)(100 * Math.random());
      int r2 = (int)(100 * Math.random());
      System.out.println(r1 + r2 + r3);
    }
  }