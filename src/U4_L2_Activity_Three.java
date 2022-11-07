import java.util.Scanner;

public class U4_L2_Activity_Three {
    public static void main(String[] args) {
        final Scanner s = new Scanner(System.in);

        boolean ok = true;

        double lo = 0;
        double la = 0;

        double mla = -90;
        double nla = 90;
        double mlo = -180; // milo :D
        double nlo = 180;

        while (ok) {
            System.out.println("Please enter the longitude:");
            lo = s.nextDouble();

            System.out.println("Please enter the latitude:");
            la = s.nextDouble();

            if (la < -90 || la > 90 || lo < -180 || lo > 180) { // obo err :(
                System.out.println("Incorrect Latitude or Longitude");
            } else {
                if (la > mla) {
                    mla = la;
                }

                if (la < nla) {
                    nla = la;
                }

                if (lo > mlo) {
                    mlo = lo;
                }

                if (lo < nlo) {
                    nlo = lo;
                }
            }

            System.out.println("Would you like to enter another location?");
            ok = s.nextInt() == 1;
        }

        s.close();

        System.out.println("Farthest North: " + mla);
        System.out.println("Farthest South: " + nla);
        System.out.println("Farthest East: " + mlo);
        System.out.println("Farthest West: " + nlo);

    }
}
