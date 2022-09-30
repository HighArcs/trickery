import java.util.Scanner;

import assignment2.Airplane;

public class Assignment2 {
    public static void main(String[] args) {
        // create a default Airplane
        final Airplane airplane1 = new Airplane();
        // create an Airplane with specific parameters
        final Airplane airplane2 = new Airplane("AAA02", 15.8f, 128, 30_000);

        final Scanner s = new Scanner(System.in);

        // ask for user input on airplane3
        System.out.println("Enter the details of the third airplane (call-sign, distance, bearing and altitude):");
        final String callSign = s.nextLine();
        final double distance = s.nextDouble();
        final int direction = s.nextInt();
        final int altitude = s.nextInt();

        s.close();

        final Airplane airplane3 = new Airplane(callSign.toUpperCase(), distance, direction, altitude);

        System.out.print("\n");

        // initial position
        System.out.println("Initial Positions:");
        System.out.println("\"Airplane 1\": " + airplane1);
        System.out.println("\"Airplane 2\": " + airplane2);
        System.out.println("\"Airplane 3\": " + airplane3);

        System.out.print("\n");

        System.out.println("Initial Distances:");
        System.out.println(
                "The distance between Airplane 1 and Airplane 2 is " + airplane1.distTo(airplane2) + " miles.");
        System.out.println(
                "The distance between Airplane 1 and Airplane 3 is " + airplane1.distTo(airplane3) + " miles.");
        System.out.println(
                "The distance between Airplane 2 and Airplane 3 is " + airplane2.distTo(airplane3) + " miles.");

        System.out.print("\n");

        System.out.println("Initial Height Differences:");
        System.out.println("The difference in height between Airplane 1 and Airplane 2 is "
                + Math.abs(airplane1.getAlt() - airplane2.getAlt()) + " feet.");
        System.out.println("The difference in height between Airplane 1 and Airplane 3 is "
                + Math.abs(airplane1.getAlt() - airplane3.getAlt()) + " feet.");
        System.out.println("The difference in height between Airplane 2 and Airplane 3 is "
                + Math.abs(airplane2.getAlt() - airplane3.getAlt()) + " feet.");

        System.out.print("\n");

        // increase altitude of airplane1 by 3,000 ft
        airplane1.gainAlt();
        airplane1.gainAlt();
        airplane1.gainAlt();

        // decrease altitude of airplane2 by 2,000 ft
        airplane2.loseAlt();
        airplane2.loseAlt();

        // decrease altitude of airplane3 by 4,000 ft
        airplane3.loseAlt();
        airplane3.loseAlt();
        airplane3.loseAlt();
        airplane3.loseAlt();

        // move airplane1 a distance that is equal to the distance between airplane2
        // and airplane3 on a heading of 65°
        final double a2a3d = airplane2.distTo(airplane3);
        airplane1.move(a2a3d, 65);

        // move airplane2 a distance of 8.0 miles on a heading of 135°
        airplane2.move(8.0, 135);

        // move airplane3 a distance of 5.0 miles on a heading of 55°
        airplane3.move(5.0, 55);

        // print new positions
        System.out.println("New Positions:");
        System.out.println("\"Airplane 1\": " + airplane1);
        System.out.println("\"Airplane 2\": " + airplane2);
        System.out.println("\"Airplane 3\": " + airplane3);

        System.out.print("\n");

        System.out.println("New Distances: ");
        System.out.println(
                "The distance between Airplane 1 and Airplane 2 is " + airplane1.distTo(airplane2) + " miles.");
        System.out.println(
                "The distance between Airplane 1 and Airplane 3 is " + airplane1.distTo(airplane3) + " miles.");
        System.out.println(
                "The distance between Airplane 2 and Airplane 3 is " + airplane2.distTo(airplane3) + " miles.");

        System.out.print("\n");

        System.out.println("New Height Differences:");
        System.out.println("The difference in height between Airplane 1 and Airplane 2 is "
                + Math.abs(airplane1.getAlt() - airplane2.getAlt()) + " feet.");
        System.out.println("The difference in height between Airplane 1 and Airplane 3 is "
                + Math.abs(airplane1.getAlt() - airplane3.getAlt()) + " feet.");
        System.out.println("The difference in height between Airplane 2 and Airplane 3 is "
                + Math.abs(airplane2.getAlt() - airplane3.getAlt()) + " feet.");
    }
}
