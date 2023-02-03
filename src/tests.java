import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collections;
import java.util.List;
import java.util.Scanner;

import shapes.Rectangle;
import shapes.RegularPolygon;

public class tests {
  final public static int[] list = { 1, 3, 5, 6, 7, 8, 11, 13, 14, 15, 16, 17, 20, 22, 24, 29, 30, 31, 33, 35 };

  public static int mystery(int[] a, int s) {
    for (int i = 0; i < a.length; i++) {
      if (a[i] == s) {
        return i;
      }
    }

    return -1;
  }

  public static void main(String[] args) {
    double[] list = { 3.5, 4.8, 2.1, 4.2 };
    System.out.println(insertSort(list));
  }

  public static int insertSort(ArrayList<String> list) {
    int count = 0;

    for (int i = 1; i < list.size(); i++) {
      String toInsert = list.get(i);
      int j;

      for (j = i; j > 0; j--) {
        count++;
        if (toInsert.compareTo(list.get(j - 1)) >= 0) {
          break;
        }
      }

      list.add(j, list.remove(i));
    }

    return count;
  }

  public static int selectSort(double[] arr) {
    int count = 0;
    for (int i = 0; i < arr.length - 1; i++) {
      int place = i;
      for (int j = i + 1; j < arr.length; j++) {
        count++;
        if (arr[j] < arr[place]) {
          place = j;
        }
      }
      double temp = arr[i];
      arr[i] = arr[place];
      arr[place] = temp;
    }
    return count;
  }

  public static int insertSort(double[] arr) {
    int count = 0;
    for (int i = 1; i < arr.length; i++) {
      double val = arr[i];
      int j;

      for (j = i - 1; j >= 0; j--) {
        count++;
        if (arr[j] > val) {
          arr[j + 1] = arr[j];
        } else {
          break;
        }
      }

      arr[j + 1] = val;
    }

    return count;
  }
}

class Light {
}

class Whatchamacallit {
  private double price;
  private String title;

  public Whatchamacallit() {
    this(0, "none");
  }

  public Whatchamacallit(double p, String t) {
    price = 0;
    if (p > 0) {
      price = p;
    }
    title = t;
  }

  public String toString() {
    return title + " costs $" + price;
  }
}