import java.util.Scanner;
import shapes.*;

public class U2_L7_Activity_Three {
    public static void main(String[] args) {
        Scanner s = new Scanner(System.in);

        Integer sides;
        Double length;

        System.out.println("Enter number of sides:");
        sides = s.nextInt();

        System.out.println("Enter side length:");
        length = s.nextDouble();

        s.close();

        final RegularPolygon p1 = new RegularPolygon(sides, length);
        final RegularPolygon p2 = new RegularPolygon(p1.getNumSides() + 1, p1.getSideLength() * 2);

        System.out.println("The area of a " + p1 + " is: " + p1.getArea());
        System.out.println("The area of a " + p2 + " is: " + p2.getArea());

    }
}
