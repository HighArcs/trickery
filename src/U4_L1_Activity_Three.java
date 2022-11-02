import java.util.Scanner;

public class U4_L1_Activity_Three {
    public static void main(String[] args) {
        final Scanner s = new Scanner(System.in);

        System.out.println("Input a word:");
        String word = s.nextLine();

        s.close();

        int i = 0;
        while (i < word.length()) {
            if (i == 0 || i % 3 != 2) {
                System.out.print(word.substring(i, i + 1));
            }

            i++;
        }
    }
}
