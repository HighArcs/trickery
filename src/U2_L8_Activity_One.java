import testing.Math;
import java.util.Scanner;

public class U2_L8_Activity_One {
    public static void main(String[] args) {
        Scanner s = new Scanner(System.in);

        System.out.println("Enter a positive integer");
        final int min = 2;
        final int max = s.nextInt() + 2;

        s.close();

        System.out.println((int) (Math.random() * (max - min + 1) + min));
        System.out.println((int) (Math.random() * (max - min + 1) + min));
        System.out.println((int) (Math.random() * (max - min + 1) + min));
    }
}
