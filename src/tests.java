import java.util.Arrays;

public class tests {

  public static void main(String[] args) {
    SuperHero superBob = new SuperHero();

    superBob.speak();
  }

  public static <T> void str(T[][] a) {
    for (T[] ts : a) {
      for (T t : ts) {
        System.out.print(t.toString() + " ");
      }

      System.out.println();
    }
  }

  public static void str(int[][] a) {
    for (int[] ts : a) {
      for (int t : ts) {
        System.out.print(t + " ");
      }

      System.out.println();
    }
  }

}

class Human {
  public Human() {
    System.out.print("I can run");
  }

  public void speak() {
    System.out.print("I can also talk");
  }
}

class SuperHero extends Human {
  public SuperHero() {
    System.out.print(" and also fly");
  }

  public void speak() {
    System.out.print("I can now talk faster");
  }
}