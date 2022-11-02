import java.util.Scanner;

public class U4_L1_Activity_Two {
    public static void main(String[] args) {
        System.out.println("Enter the Scores:");
        final Scanner s = new Scanner(System.in);

        int largest = -Integer.MIN_VALUE;

        while (true) {
            int n = s.nextInt();
            if (n == -1) {
                break;
            }

            if (n > largest) {
                largest = n;
            }
        }

        s.close();

        System.out.println("The largest score is " + largest);
    }
}
