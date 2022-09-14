import java.util.Scanner;

public class U2_L1_Activity_One {
    public static void main(String[] args) {
        final Scanner s = new Scanner(System.in);

        System.out.println("What is your name?");

        final String name = s.nextLine();

        System.out.println("What is your favorite number?");

        final int n = s.nextInt();

        s.close();

        System.out.println("Your name is " + name + " and you like the number " + n + ".");
    }
}
