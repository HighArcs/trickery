import java.util.Scanner;

public class U3_L5_Activity_One {
    public static void main(String[] args) {

        final Scanner s = new Scanner(System.in);

        System.out.println("Enter 2 integers:");
        final double x = s.nextInt(); // int -> final double
        final double y = s.nextInt(); // flip contents, int -> final double and nextDouble() -> nextInt()

        s.close();

        final double ratio = x / y;
        if (ratio > 1 && ratio <= 8) { // rewrite if statement
            System.out.println("Ratio OK");
        }

    }
}
