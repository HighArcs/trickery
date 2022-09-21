import java.util.Scanner;
import shapes.*;

public class U2_L5_Activity_Three {
    public static void main(String[] args) {
        final Scanner s = new Scanner(System.in);

        System.out.println("Type a side length:");
        final double l = s.nextDouble();

        s.close();

        System.out.println(new RegularPolygon(3, l));
        System.out.println(new RegularPolygon(4, l));
        
    }
}
