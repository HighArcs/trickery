import java.util.Scanner;

public class U2_L8_Activity_Two {
    public static void main(String[] args) {
        final Scanner s = new Scanner(System.in);

        System.out.println("Enter the first x-coordinate:");
        final double x1 = s.nextDouble();

        System.out.println("Enter the second x-coordinate:");
        final double x2 = s.nextDouble();

        System.out.println("Enter the first y-coordinate:");
        final double y1 = s.nextDouble();

        System.out.println("Enter the second y-coordinate:");
        final double y2 = s.nextDouble();

        s.close();

        final double dy = y2 - y1;
        final double dx = x2 - x1;
        final double slope = dy / dx;

        System.out.println("Slope: " + slope);
    }
}
