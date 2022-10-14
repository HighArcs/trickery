import java.util.Scanner;

public class U3_L5_Activity_Two {
    public static void main(String[] args) {

        final Scanner s = new Scanner(System.in);
        System.out.println("Enter two numbers:");

        final int a = s.nextInt(); // int -> final int, nextDouble() -> nextInt()
        final int b = s.nextInt(); // int -> final int

        s.close();

        if (b != 0 && a % b == 0) {
            System.out.println(b + " is a factor of " + a);
        } else {
            System.out.println(b + " is not a factor of " + a);
        }
    }
}
