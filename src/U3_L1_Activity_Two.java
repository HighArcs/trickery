import java.util.Scanner;

public class U3_L1_Activity_Two {
    public static void main(String[] args) {
        final Scanner s = new Scanner(System.in);

        System.out.println("Please enter an integer:");
        final int i32 = s.nextInt();

        s.close();

        if (i32 == 48) {
            System.out.println("YES");
        }
    }
}
