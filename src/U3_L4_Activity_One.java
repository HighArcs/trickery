import java.util.Scanner;

public class U3_L4_Activity_One {
    public static void main(String[] args) {
        final Scanner s = new Scanner(System.in);

        System.out.println("Enter a number:");
        final int i32 = s.nextInt();

        s.close();

        if (i32 > 100 || i32 < 65) {
            System.out.println("True");
        } else {
            System.out.println("False");
        }
    }
}
