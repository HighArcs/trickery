import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collections;

public class tests {

  private static int incr = 0;

  public static void main(String[] args) {
    int[] a = {};
    System.out.println(a[0]);
  }

  private static final ArrayList<Integer> vals = new ArrayList<>();

  public static void binaryInsert(int num) {
    int low = 0;
    int high = vals.size();

    while (high > low) {
      int mid = (low + high) / 2;
      System.out.println("mid: " + mid + ", low: " + low + ", high: " + high);
      if (vals.get(mid).intValue() < num) {
        low = mid + 1;
      } else {
        high = mid;
      }

    }

    vals.add(low, new Integer(num));
  }

  public static <T> void str(T[][] a) {
    for (T[] ts : a) {
      for (T t : ts) {
        System.out.print(t.toString() + " ");
      }

      System.out.println();
    }
  }

  private static int[] nums = { 0, 3, 1, 1, 1, 6, 6 };

  public int findRepeatIndex() {
    return repeatHelper(nums.length - 1);
  }

  private static int repeatHelper(int prev) {
    if (prev >= nums.length - 1) {
      return -1;
    } else if (nums[prev] == nums[prev + 1]) {
      return prev;
    } else {
      return repeatHelper(prev + 1);
    }
  }

  public static int recur(int x) {
    if (x > 15) {
      return x / 2;
    } else {
      return recur(recur(x * 3));
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