import java.util.Scanner;

public class U4_L3_Activity_Three {
    public static void main(String[] args) {
        final Scanner s = new Scanner(System.in);

        System.out.println("Enter a number between 0 and 50:");
        final int i32 = s.nextInt();

        s.close();

        if (i32 <= 0 || i32 >= 50) {
            System.out.println("error");
        } else {
            for (int i = i32; i <= 50; i++) {
                if (i % 5 == i32 % 5) {
                    System.out.print("\n");
                }

                System.out.print(i + " ");
            }
        }
    }
}
