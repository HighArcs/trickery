import java.util.Scanner;

class U1_L2_Activity_Three {
    public static void main(String[] args) {
        final Scanner s = new Scanner(System.in);

        System.out.println("Hi there. What is your name?");
        final String name = s.nextLine();

        System.out.println("What state do you live in?");
        final String location = s.nextLine();

        s.close();

        System.out.println("My name is " + name + ". I live in " + location + ".");
    }
}
