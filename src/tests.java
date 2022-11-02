import java.util.Scanner;

public class tests {
    public static void main(String[] args) {
        int a = 66, b = 25, f = 0, d = 2;
        while (d <= a) {
            if (a % d == 0 && b % d == 0) {
                f = 1;
            }

            d++;
        }

        System.out.println(f);
    }
}
