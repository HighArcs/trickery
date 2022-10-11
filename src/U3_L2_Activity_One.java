import java.util.Scanner;

public class U3_L2_Activity_One {
    public static void main(String[] args) {
        final Scanner s = new Scanner(System.in);

        System.out.println("Please enter five numbers:");

        final double a = s.nextDouble();
        final double b = s.nextDouble();
        final double c = s.nextDouble();
        final double d = s.nextDouble();
        final double e = s.nextDouble();

        s.close();

        final double average = (a + b + c + d + e) / 5;

        if (average >= 89.95) {
            System.out.println("ABOVE AVERAGE");
        }
    }
}
