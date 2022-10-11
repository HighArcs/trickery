import java.util.Scanner;

public class U2_L3_Activity_One {
    public static void main(String[] args) {
        final Scanner s = new Scanner(System.in);

        System.out.println("Enter a string:");
        final String content = s.nextLine();

        System.out.println("Enter a number:");
        final int location = s.nextInt();

        s.close();

        System.out.println(content.substring(0, location) + content.substring(content.length() - location));
    }
}
