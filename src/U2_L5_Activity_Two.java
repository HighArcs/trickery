import java.util.Scanner;
import shapes.*;

public class U2_L5_Activity_Two {
    public static void main(String[] args) {
        final Scanner s = new Scanner(System.in);

        System.out.println("Type a number for length and width:");
        final double lw = s.nextDouble();

        System.out.println("Type a length:");
        final double l = s.nextDouble();

        System.out.println("Type a width:");
        final double w = s.nextDouble();

        s.close();

        System.out.println(new Rectangle(lw));
        System.out.println(new Rectangle(l, w));

    }
}
