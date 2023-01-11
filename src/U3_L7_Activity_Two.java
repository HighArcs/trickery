import java.util.Scanner;

public class U3_L7_Activity_Two {
    public static void main(String[] args) {
        final Scanner s = new Scanner(System.in);

        System.out.println("Enter 2 strings:");
        final String a = s.nextLine();
        final String b = s.nextLine();

        s.close();

        if (a.equals(b)) {
            System.out.println("Equal!");
        } else if (a.equalsIgnoreCase(b)) {
            System.out.println("Different case");
            // short circuit with length
        } else if (a.length() == b.length()
                && a.substring(0, a.length() - 1).equals(b.substring(0, b.length() - 1))) {
            System.out.println("Close enough");
        } else {
            System.out.println("Try again!");
        }
    }
}
