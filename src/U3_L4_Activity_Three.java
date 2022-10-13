// something wrong?

import java.util.Scanner;

public class U3_L4_Activity_Three {
    public static void main(String[] args) {
        final Scanner s = new Scanner(System.in);

        System.out.println("Please enter the latitude:");
        final double latitude = s.nextDouble();

        System.out.println("Please enter the longitude:");
        final double longitude = s.nextDouble();

        s.close();

        if (latitude > 90 || latitude < -90) {
            System.out.println("latitude is incorrect");
        }

        if (longitude > 180 || latitude < -180) {
            System.out.println("longitude is incorrect");
        }

        if (latitude <= 90 && latitude >= -90 && longitude <= 180 && longitude >= -180) {
            System.out.println("The location: " + latitude + ", " + longitude);
        }
    }
}
