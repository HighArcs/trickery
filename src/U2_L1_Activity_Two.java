import java.util.Scanner;

public class U2_L1_Activity_Two {
    public static void main(String[] args) {

        String order = new String("apple pie");

        System.out.println("The current order is " + order);

        System.out.println("I want to eat something else, what do you want to eat?");

        final Scanner s = new Scanner(System.in);

        order = s.nextLine();

        s.close();

        System.out.println("The order has changed to " + order);
    }
}
