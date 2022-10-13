import java.util.Scanner;

public class U3_L3_Activity_Four {
    public static void main(String[] args) {
        final Scanner s = new Scanner(System.in);

        System.out.println("What is the temperature?");
        final int temperature = s.nextInt();

        s.close();

        if (temperature >= 97) {
            if (temperature <= 99) {
                System.out.println("Temperature is OK");
            } else {
                System.out.println("NOT NORMAL");
            }
        } else {
            System.out.println("NOT NORMAL");
        }
    }
}
