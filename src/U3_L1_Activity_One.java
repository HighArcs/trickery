import java.util.Scanner;

public class U3_L1_Activity_One {
    public static void main(String[] args) {
        final Scanner s = new Scanner(System.in);

        System.out.println("Please enter a double:");
        double d = s.nextDouble();

        s.close();

        if (d == 12.345) {
            System.out.println("YES");
        }
    }
}
