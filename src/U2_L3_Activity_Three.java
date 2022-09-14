import java.util.Scanner;

public class U2_L3_Activity_Three {
    public static void main(String[] args) {
        final Scanner s = new Scanner(System.in);

        System.out.println("Enter first word:");

        final String a = s.nextLine().toLowerCase();

        System.out.println("Enter second word:");

        final String b = s.nextLine().toLowerCase();

        s.close();

        System.out.println("Result: " + a.compareTo(b));
    }
}
