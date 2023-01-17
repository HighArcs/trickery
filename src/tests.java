import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.Scanner;

import shapes.Rectangle;
import shapes.RegularPolygon;

public class tests {
  public static void main(String[] args) {
    Rectangle[] rectangles = { new Rectangle(4, 1), new Rectangle(2, 5), new Rectangle(7, 6) };
    for (Rectangle r : rectangles) {
      if (r.getPerimeter() > 12) {
        r.setLength(3);
      } else {
        r = new Rectangle(1, 2);
      }
    }
    for (Rectangle r : rectangles) {
      System.out.print(r.getLength() + " ");
    }
  }
}