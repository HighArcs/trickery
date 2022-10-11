import java.util.Scanner;

public class U3_L2_Activity_Two {
    public static void main(String[] args) {
        final Scanner s = new Scanner(System.in);

        System.out.println("Please enter two numbers:");

        final double a = s.nextDouble();
        final double b = s.nextDouble();

        s.close();

        if (a <= b) {
            System.out.println("Smallest is: " + a);
        }

        if (b < a) {
            System.out.println("Smallest is: " + b);
        }

        // alternative solution:
        // System.out.println("Smallest is: " + Math.min(a, b));
    }
}
