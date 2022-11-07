import java.util.Scanner;

public class U4_L4_Activity_One {
    public static void main(String[] args) {
        final Scanner s = new Scanner(System.in);

        System.out.println("Input String:");
        final String str = s.nextLine();

        s.close();

        int count = 0;
        for (int i = 0; i < str.length() - 1; i++) {
            final String first = str.substring(i, i + 1).toLowerCase();
            final String second = str.substring(i+1, i + 2).toLowerCase();

            if (first.equals("p") && (second.equals("a")
                    || second.equals("e") || second.equals("i")
                    || second.equals("o") || second.equals("u"))) {
                count++;
            }
        }

        System.out.println("Contains p followed by a vowel " + count + " times.");
    }
}
