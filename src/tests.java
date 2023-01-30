import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collections;
import java.util.List;
import java.util.Scanner;

import shapes.Rectangle;
import shapes.RegularPolygon;

public class tests {
  public static void repeatsAway(ArrayList<Integer> list) {
    for (int i = list.size() - 1; i > 0; i--) {
      if (list.get(i).equals(list.get(i - 1))) {
        list.remove(i);
      }
    }
  }

  public static void main(String[] args) {
    ArrayList<Integer> a = new ArrayList<Integer>();
    a.add(2);
    a.add(2);
    a.add(7);
    a.add(5);
    a.add(7);
    a.add(7);
    a.add(7);
    repeatsAway(a);
    System.out.println(a);
  }

}