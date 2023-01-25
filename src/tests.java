import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.Scanner;

import shapes.Rectangle;
import shapes.RegularPolygon;

public class tests {
  public static void main(String[] args) {
    ArrayList<String> words = new ArrayList<String>();
    words.add("set");
    words.add("interface");
    words.add("limit");
    words.add("testify");
    words.add(2, "disagree");
    System.out.println(words);
  }

  public static int checkString(String[] arr) {
    int count = 0;
    for (int k = 0; k < arr.length; k++) {
      if (arr[k].length() >= 3) {
        count++;
      }
    }

    return count;
  }
}