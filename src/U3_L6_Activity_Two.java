import java.util.Scanner;

public class U3_L6_Activity_Two {
    public static void main(String[] args) {

        final Scanner s = new Scanner(System.in);
        final int x = s.nextInt();
        final int y = s.nextInt();

        s.close();

        if (y <= 9 || (x > 2 && x * y > 10)) { // rewrite if statement
            System.out.println("pass");
        }

    }
}
