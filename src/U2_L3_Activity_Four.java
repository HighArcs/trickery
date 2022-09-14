import java.util.Scanner;

public class U2_L3_Activity_Four {
    public static void main(String[] args) {
        final Scanner s = new Scanner(System.in);

        System.out.println("Enter a sentence:");

        final String content = s.nextLine();

        s.close();

        final String word = content.split(" ")[0];

        System.out.println(word.length());
    }
}