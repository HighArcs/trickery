import java.util.Scanner;

public class U4_L4_Activity_Three {
    public static void main(String[] args) {
        final Scanner s = new Scanner(System.in);

        System.out.println("Enter two strings:");
        final String a = s.nextLine();
        final String b = s.nextLine();

        s.close();

        if (a.length() != b.length()) {
            System.out.println("error");
        } else {
            for (int i = a.length() - 1; i >= 0; i--) {
                System.out.print(b.charAt(i));
                System.out.print(a.charAt(i));
            }
        }
    }
}
