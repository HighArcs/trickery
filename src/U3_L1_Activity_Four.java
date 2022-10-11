import java.util.Scanner;

public class U3_L1_Activity_Four {
    public static void main(String[] args) {
        final Scanner s = new Scanner(System.in);

        System.out.println("Please enter an integer:");
        final int i32 = s.nextInt();

        s.close();

        if (i32 % 2 == 0) {
            System.out.println("Divisible by 2!");
        }

        if (i32 % 3 == 0) {
            System.out.println("Divisible by 3!");
        }
    }
}
