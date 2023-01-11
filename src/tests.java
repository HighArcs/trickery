import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.Scanner;

public class tests {
  public static void main(String[] args) {

    int[] arr = { 7, 4, 8, 12, 1 };
    delete(arr, 2);

    System.out.println(Arrays.toString(arr));
  }

  public static void insert(int[] nums, int val, int pos) {
    for (int i = nums.length - 1; i > pos; i--) {
      nums[i] = nums[i - 1];
    }

    nums[pos] = val;
  }

  public static void delete(int[] nums, int pos) {
    for (int i = pos; i < nums.length - 1; i++) {
      nums[i] = nums[i + 1];
    }
  }
}