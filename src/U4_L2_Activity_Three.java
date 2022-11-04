// literally no idea how to fix this

import java.util.Scanner;

public class U4_L2_Activity_Three {
    public static void main(String[] args) {
        final Scanner s = new Scanner(System.in);

        double lat_min = 90;
        double lat_max = -90;

        double lon_min = 180;
        double lon_max = -180;

        boolean going = true;

        while (going) {
            System.out.println("Please enter the latitude:");
            double lat = s.nextDouble();

            System.out.println("Please enter the longitude:");
            double lon = s.nextDouble();

            if (lat < -90 || lat > 90 || lon < -180 || lon > 180) {
                System.out.println("Incorrect longitude or latitude");
                break;
            }

            if (lat > lat_max) {
                lat_max = lat;
            }
            if (lat < lat_min) {
                lat_min = lat;
            }

            if (lon > lon_max) {
                lon_max = lon;
            }
            if (lon < lon_min) {
                lon_min = lon;
            }

            System.out.println("Would you like to enter another location?");

            if (s.nextInt() == 0) {
                going = false;
                System.out.println("Farthest North: " + lat_max);
                System.out.println("Farthest South: " + lat_min);
                System.out.println("Farthest West: " + lon_max);
                System.out.println("Farthest East: " + lon_min);
            }
        }

        s.close();

    }
}
