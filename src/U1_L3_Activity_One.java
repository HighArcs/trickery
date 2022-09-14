import java.util.Scanner;

class U1_L3_Activity_One {
    public static void main(String[] args) {
        System.out.println("Print 3 doubles:");

        final Scanner s = new Scanner(System.in);

        final double a = s.nextDouble();
        final double b = s.nextDouble();
        final double c = s.nextDouble();

        s.close();

        System.out.println(c + " " + b + " " + a);
    }
}
