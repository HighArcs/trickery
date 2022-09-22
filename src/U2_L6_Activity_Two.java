import java.util.Scanner;
import shapes.*;

public class U2_L6_Activity_Two {
    public static void main(String[] args) {
        final Scanner s = new Scanner(System.in);

        System.out.println("Enter number of sides:");
        final int sides = s.nextInt();

        System.out.println("Enter the side length:");
        final double length = s.nextDouble();

        s.close();
        final RegularPolygon polygon = new RegularPolygon(sides, length);

        System.out.println("Area with " + polygon.getNumSides() + " sides: " + polygon.getArea());

        System.out.println("Incrementing the number of sides by two");
        polygon.addSides(2);

        System.out.println("Area with " + polygon.getNumSides() + " sides: " + polygon.getArea());
    }
}
