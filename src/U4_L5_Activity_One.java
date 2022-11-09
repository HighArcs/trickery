import java.util.Scanner;

public class U4_L5_Activity_One {
    public static void main(String[] args) {
        final Scanner s = new Scanner(System.in);

        System.out.println("Input a String:");
        final String str = s.nextLine();

        System.out.println("Input an integer");
        final int i32 = s.nextInt();

        s.close();

        for (int i = 0; i < str.length(); i++) {
            for (int j = 0; j < i32; j++) {
                System.out.print(str.charAt(i));
            }
        }
    }
}
