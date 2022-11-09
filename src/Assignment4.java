import java.util.Scanner;

public class Assignment4 {
    public static void main(String[] args) {
        final Scanner s = new Scanner(System.in);

        System.out.println("Type the message to be shortened");
        final String str = s.nextLine().toLowerCase();

        s.close();

        System.out.println();

        System.out.println("Algorithm 1");

        int v1 = 0;
        int r1 = 0;
        String message1 = "";

        for (int i = 0; i < str.length() - 1; i++) {
            final char f = str.charAt(i);
            final char r = str.charAt(i + 1);

            // i != 0 -> not beginning of message
            // f == '?' -> is some vowel
            // charAt(i - 1) != ' ' -> previous char is not space
            if (i != 0 && (f == 'a' || f == 'e' || f == 'i' || f == 'o' || f == 'u') && (str.charAt(i - 1) != ' ')) {
                v1++;
                continue;
            }

            if (f == r) {
                r1++;
                message1 += f;
                i++;
                continue;
            }

            message1 += f;
        }

        System.out.println("Vowels removed: " + v1);
        System.out.println("Repeats removed: " + r1);
        System.out.println("Algorithm 1 message: " + message1);
        System.out.println("Algorith 1 characters saved: " + (str.length() - message1.length()));
    }
}
