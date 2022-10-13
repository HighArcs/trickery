import java.util.Scanner;

public class U3_L3_Activity_Two {
    public static void main(String[] args) {
        final Scanner s = new Scanner(System.in);

        System.out.println("Please enter a letter grade:");
        final String letter = s.nextLine();

        s.close();

        if (letter.equals("A")) {
            System.out.println("90-100");
        } else if (letter.equals("B")) {
            System.out.println("80-90");
        } else if (letter.equals("C")) {
            System.out.println("70-80");
        } else if (letter.equals("D")) {
            System.out.println("60-70");
        } else if (letter.equals("F")) {
            System.out.println("0-60");
        } else {
            System.out.println("Invalid letter grade");
        }
    }
}
