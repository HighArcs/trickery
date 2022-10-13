import java.util.Scanner;

public class U3_L4_Activity_Two {
    public static void main(String[] args) {
        final Scanner s = new Scanner(System.in);

        System.out.println("Enter two numbers:");
        final int a = s.nextInt();
        final int b = s.nextInt();

        s.close();

        if (a >= 0 && a % 2 == 0 && b >= 0 && b % 2 == 0) {
            System.out.println("Both are positive and even.");
        } else {
            System.out.println("At least one is negative or odd.");
        }
    }
}
