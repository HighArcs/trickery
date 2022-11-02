import java.util.Scanner;

public class U4_L2_Activity_Three {
    public static void main(String[] args) {
        final Scanner s = new Scanner(System.in);

        double latmin = Double.MAX_VALUE;
        double latmax = -Double.MAX_VALUE;

        double lonmin = Double.MAX_VALUE;
        double lonmax = -Double.MAX_VALUE;

        while (true) {

            System.out.println("Please enter the latitude:");
            double lat = s.nextDouble();

            System.out.println("Please enter the longitude:");
            double lon = s.nextDouble();

            if (lat > latmax) { latmax = lat; }
            if (lat < latmin) { latmax = lat; }

            if (lon > lonmax) { lonmax = lon; }
            if (lon < lonmin) { lonmin = lon; }
            

            System.out.println("Would you like to enter another location?");

            if (s.nextInt() == 0) {
                break;
            }
        }

        s.close();
    }
}
