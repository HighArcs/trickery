import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collections;
import java.util.List;
import java.util.Scanner;

import shapes.Rectangle;
import shapes.RegularPolygon;

public class tests {
  public static void mystery(ArrayList<Integer> list1, ArrayList<Integer> list2) {
    for (int i = 0; i < list1.size(); i++) {
      if (list1.get(i) > list2.get(i)) {
        System.out.print(list1.get(i));
      } else {
        System.out.print(list2.get(i));
      }
      System.out.print(" ");
    }
  }

  public static void main(String[] args) {
    ArrayList<Integer> nums1 = new ArrayList<Integer>();
    nums1.add(7);
    nums1.add(4);
    nums1.add(5);
    nums1.add(12);

    ArrayList<Integer> nums2 = new ArrayList<Integer>();
    nums2.add(5);
    nums2.add(6);
    nums2.add(2);
    nums2.add(11);

    mystery(nums1, nums2);
  }

}