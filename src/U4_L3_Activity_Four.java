import java.util.Scanner;

public class U4_L3_Activity_Four {
    public static void main(String[] args) {
        final Scanner s = new Scanner(System.in);

        System.out.println("Enter a positive integer:");
        final int i32 = s.nextInt();

        s.close();

        if (i32 <= 0) {
            System.out.println("error");
        } else {
            for (int i = i32; i >= 0; i--) {
                if (i % 3 == 0) {
                    System.out.print(i + " ");
                }
            }
        }

    }
}
