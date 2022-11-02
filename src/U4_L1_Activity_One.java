import java.util.Scanner;

public class U4_L1_Activity_One {
    public static void main(String[] args) {
        System.out.println("Enter any numbers (Enter 5 to stop)");
        final Scanner s = new Scanner(System.in);

        int sum = 0;
        
        while (true) {
            int n = s.nextInt();
            if (n == 5) {
                break;
            }

            sum += n;
        }

        s.close();

        System.out.println("Sum is " + sum);
    }
}
