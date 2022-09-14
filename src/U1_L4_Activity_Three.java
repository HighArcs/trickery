import java.util.Scanner;

class U1_L4_Activity_Three {
    public static void main(String[] args) {

        Scanner s = new Scanner(System.in);

        double d = s.nextDouble();

        s.close();

        double pi = 3.14;
        double tau = 6.28;

        double r = d / tau;

        System.out.println(String.format("Radius: %f", r));
        System.out.println(String.format("Area: %f", pi * r * r));

    }
}