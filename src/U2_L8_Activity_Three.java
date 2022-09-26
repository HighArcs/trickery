import java.util.Scanner;

public class U2_L8_Activity_Three {
    public static void main(String[] args) {
        final Scanner s = new Scanner(System.in);

        System.out.println("Enter two doubles:");

        final double x = s.nextDouble();
        final double y = s.nextDouble();

        s.close();

        int d = (int) Math.round(Math.abs(x - y));

        System.out.println("Difference: " + d);
    }
}
