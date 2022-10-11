
import java.util.Scanner;

class U1_L2_Activity_Two {
    public static void main(String[] args) {
        final Scanner s = new Scanner(System.in);

        System.out.println("Please enter four names:");

        final String a = s.nextLine();
        final String b = s.nextLine();
        final String c = s.nextLine();
        final String d = s.nextLine();

        s.close();

        System.out.println(d + " " + b + " " + c + " " + a);
    }
}
