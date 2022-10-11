import java.util.Scanner;

public class U2_L3_Activity_Two {
    public static void main(String[] args) {
        final Scanner s = new Scanner(System.in);

        System.out.println("Enter a string:");
        final String content = s.nextLine();

        System.out.println("How many characters would you like to delete at the end?");
        final int amount = s.nextInt();

        s.close();

        System.out.println(content.substring(0, content.length() - amount));
    }
}
