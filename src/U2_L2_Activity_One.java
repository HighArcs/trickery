import java.util.Scanner;

public class U2_L2_Activity_One {
    public static void main(String[] args) {

        final Scanner s = new Scanner(System.in);

        System.out.println("What type of item are you buying?");
        final String item = s.nextLine();

        System.out.println("How many are you buying?");
        final int count = s.nextInt();

        System.out.println("How much does each one weigh?");
        final double weight = s.nextDouble();

        s.close();

        System.out.println("" + count + " " + item + " at " + weight + " pounds each will weigh "
                + ((double) count * weight) + " pounds total");

    }
}