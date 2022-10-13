import java.util.Scanner;

public class U3_L3_Activity_Three {
    public static void main(String[] args) {
        final Scanner s = new Scanner(System.in);

        System.out.println("Please enter two integers:");
        final int a = s.nextInt();
        final int b = s.nextInt();

        System.out.println(a + " + " + b + " = ?");
        final int given = s.nextInt();

        s.close();

        if (given == (a + b)) {
            System.out.println("Correct!");
        } else {
            System.out.println("Wrong");
        }

    }
}
