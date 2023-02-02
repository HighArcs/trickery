import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collections;
import java.util.List;
import java.util.Scanner;

import shapes.Rectangle;
import shapes.RegularPolygon;

public class tests {
  public static void main(String[] args) {

    Integer a = 3;
    int b = 3;
    Integer c = new Integer();
    Integer d = new Integer(3);
    int e = new int(3);
    
  }

  public static boolean mystery(int[] a) {
    boolean flag = true;
    for (int i = 1; i < a.length; i++) {
      if (a[i] < a[i - 1]) {
        flag = false;
        break;
      }
    }
    return flag;
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