import java.util.Scanner;

public class U3_L1_Activity_Three {
    public static void main(String[] args) {
        final Scanner s = new Scanner(System.in);

        System.out.println("Please enter two integers:");

        final int a = s.nextInt();
        final int b = s.nextInt();

        s.close();

        if (b == (a * 2)) {
            System.out.println("YES");
        }
    }
}
