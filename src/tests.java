import java.util.Scanner;

public class tests {
    public static void main(String[] args) {
        int x = 7;
        int y = 5;
        doStuff(x, y);
        System.out.println(x + " " + y);
    }

    public static void doStuff(int a, int b) {
        a++;
        b++;
        System.out.println(a + " " + b);
    }
}
