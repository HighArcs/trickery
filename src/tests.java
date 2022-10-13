import java.util.Scanner;

public class tests {
    public static void main(String[] args) {
        int a = 1;
        int b = 2;

        if (a <= 2 && b < a) {
            a *= 2;
        } else {
            b *= 2;
        }

        if (!(a == 3 || b == 4)) {
            a *= 3;
        } else {
            b *= 5;
        }

        System.out.println(a + " " + b);
    }
}
