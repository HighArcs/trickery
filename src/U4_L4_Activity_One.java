import java.util.Scanner;

public class U4_L4_Activity_One {
    public static void main(String[] args) {
        final Scanner s = new Scanner(System.in);

        System.out.println("Input String:");
        final String str = s.nextLine().toLowerCase();

        s.close();

        int count = 0;
        for (int i = 0; i < str.length() - 1; i++) {
            final char first = str.charAt(i);
            final char second = str.charAt(i + 1);

            if (first == 'p' && (second == 'a'
                    || second == 'e' || second == 'i'
                    || second == 'o' || second == 'u')) {
                count++;
            }
        }

        System.out.println("Contains p followed by a vowel " + count + " times.");
    }
}
