import java.util.Scanner;

public class U3_L2_Activity_Four {
    public static void main(String[] args) {
        final Scanner s = new Scanner(System.in);

        System.out.println("Enter two test scores:");

        final double a = s.nextDouble();
        final double b = s.nextDouble();

        s.close();

        if (a < 0) {
            System.out.println("Not Valid");
        }

        if (b < 0) {
            System.out.println("Not Valid");
        }

        if (a > 100) {
            System.out.println("Not Valid");
        }

        if (b > 100) {
            System.out.println("Not Valid");
        }

    }
}
