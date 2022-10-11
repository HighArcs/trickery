import java.util.Scanner;

public class U1_L3_Activity_Two {
    public static void main(String[] args) {
        final Scanner s = new Scanner(System.in);

        System.out.println("Hi there. What is your name?");
        final String name = s.nextLine();

        System.out.println("Hi " + name + ". How old are you?");
        final int age = s.nextInt();

        System.out.println(name + " is " + age + "years old.");

        s.close();
    }
}
