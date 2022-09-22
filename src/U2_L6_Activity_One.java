import java.util.Scanner;
import shapes.*;

public class U2_L6_Activity_One {
    public static void main(String[] args) {
        final Scanner s = new Scanner(System.in);

        System.out.println("Enter the radius of the circle:");
        final double radius = s.nextDouble();

        s.close();

        final Circle circle = new Circle(radius);
        final double circumference = circle.getCircumference();
        final double area = circle.getArea();

        System.out.println("A circle with a radius " + radius + " has a circumference of " + circumference
                + " and an area of " + area);
    }
}
