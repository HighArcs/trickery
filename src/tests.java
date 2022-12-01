import java.util.Scanner;

import shapes.Circle;
import shapes.Rectangle;

public class tests {
    public static void main(String[] args) {
        int a = 5;
        int b = doubleVal(a);
        System.out.print(a + " " + b);
    }

    public static int doubleVal(int n) {
        n *= 2;
        return n;
    }

}
