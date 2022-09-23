import java.util.Scanner;

public class U2_L7_Activity_Two {
    public static void main(String[] args) {

        Scanner scan = new Scanner(System.in);
        Integer x = null; // change Long to Integer
        Integer y = null; // change Long to Integer; change NaN to null

        System.out.println(x + " " + y); // add a space
        System.out.println("Enter values:");

        x = scan.nextInt(); // change nextLong to nextInt
        y = scan.nextInt();

        scan.close();

        Double avg = ((double) (x + y) / 2.0f); // move the cast
        System.out.println("Average of " + x + " and " + y + " is " + avg);

    }
}
