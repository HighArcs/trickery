import java.util.Scanner;

public class U3_L3_Activity_One {
    public static void main(String[] args) {
        final Scanner s = new Scanner(System.in);
        
        System.out.println("Please enter an integer");
        final int i32 = s.nextInt();
        
        s.close();

        if (i32 % 2 == 0) {
            System.out.println("Even");
        } else {
            System.out.println("Odd");
        }
    }
}
