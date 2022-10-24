import java.util.Scanner;

public class Assignment3 {
    public static void main(String[] args) {
        final Scanner s = new Scanner(System.in);

        System.out.println("Welcome. What is your name?");
        final String name = s.nextLine();

        System.out.println("Hello " + name + ". Are you ready to crack the code?");
        final boolean isReady = s.nextLine().equalsIgnoreCase("yes");

        if (!isReady) {
            s.close();
            return;
        }

        System.out.println("");
        System.out.println("PHASE 1");
        System.out.println("Enter a string: ");
        final String content = s.nextLine();

        if (content.length() == 3) {
            System.out.println("Correct!");

            System.out.println("");
            System.out.println("PHASE 2");
            System.out.println("Enter a number: ");
            final int i32 = s.nextInt();

            if (i32 == 19 || (i32 >= 35 && i32 < 78)) {
                System.out.println("Correct!");

                System.out.println("");
                System.out.println("PHASE 3");
                System.out.println("Enter a number: ");
                final int u32 = s.nextInt();

                if (u32 > 0 && (u32 % 2 == 0 || u32 % 10 == 1)) {
                    System.out.println("Correct!");
                    System.out.println("You have cracked the code!");
                } else {
                    s.close();
                    System.out.println("Sorry, that was incorrect!");
                    System.out.println("Better luck next time!");
                }
            } else {
                s.close();
                System.out.println("Sorry, that was incorrect!");
                System.out.println("Better luck next time!");
            }
        } else {
            s.close();
            System.out.println("Sorry, that was incorrect!");
            System.out.println("Better luck next time!");
        }

        s.close();
    }
}
