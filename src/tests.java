import java.util.Arrays;

public class tests {

  private static int incr = 0;

  public static void main(String[] args) {
    int[] elements = { 3, 4, 6, 2, 7, 7 };
    for (int j = 0; j < elements.length - 1; j++) {
      int minIndex = j;
      for (int k = j + 1; k < elements.length; k++) {
        incr++;
        if (elements[k] < elements[minIndex]) { // comparison performed
          minIndex = k;
        }
      }

      int temp = elements[j];
      elements[j] = elements[minIndex];
      elements[minIndex] = temp;
    }

    System.out.println(incr);
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