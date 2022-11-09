import java.util.Scanner;

public class U4_L4_Activity_Two {
    public static void main(String[] args) {
        final Scanner s = new Scanner(System.in);

        System.out.println("Input String:");
        final String str = s.nextLine().toLowerCase();

        s.close();

        for (int i = 0; i < str.length(); i++) {
            final char c = str.charAt(i);

            if (c != 'e' && c != 't' && c != 'a' && c != 'i' && c != 'o') {
                System.out.print(c);
            }
        }
    }
}
