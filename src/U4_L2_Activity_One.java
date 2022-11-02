import java.util.Scanner;

public class U4_L2_Activity_One {
    public static void main(String[] args) {
        final Scanner s = new Scanner(System.in);
        
        System.out.println("Enter two numbers:");
        final int a  = s.nextInt();
        final int b  = s.nextInt();
        
        s.close();

        int i = a;
        while (i <= b) {
            if (i % 2 == 1) {
                System.out.print("" + i + " ");
            }

            i++;
        }
    }
}
