import shapes.*;
import java.util.Scanner;

public class U3_L7_Activity_Three {
    public static void main(String[] args) {
        final Scanner s = new Scanner(System.in);

        System.out.println("Enter the side length of the first regular polygon:");
        final double sideLength = s.nextDouble();

        System.out.println("Enter the number of sides of the second polygon:");
        final int sideCount = s.nextInt();

        System.out.println("Enter the side length of the second polygon:");
        final double targetSideLength = s.nextDouble();

        s.close();

        final RegularPolygon source = new RegularPolygon(sideLength);
        final RegularPolygon target = new RegularPolygon(sideCount, targetSideLength);

        if (source.equals(target)) {
            System.out.println("Congruent Regular Polygons!");
        } else {
            System.out.println("Different Regular Polygons");
        }
    }
}
