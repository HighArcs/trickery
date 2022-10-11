import java.util.Scanner;

public class U3_L2_Activity_Three {
    public static void main(String[] args) {
        final Scanner s = new Scanner(System.in);

        System.out.println("What is the temperature?");
        final double temp = s.nextDouble();

        s.close();

        if (temp < 97) {
            System.out.println("NOT NORMAL");
        }

        if (temp > 99) {
            System.out.println("NOT NORMAL");
        }
    }
}
