import java.util.Scanner;

public class U4_L2_Activity_Two {
    public static void main(String[] args) {
        final Scanner s = new Scanner(System.in);

        System.out.println("Enter a positive integer:");
        int i32 = s.nextInt();

        s.close();

        if (i32 < 0) {
            System.out.println("Not a positive integer");
            return;
        }

        int p = 0;
        while (i32 > 0) {
            System.out.println((i32 % 10) * (int) Math.pow(10, p++));
            i32 /= 10;
        }

    }
}
