import java.util.Scanner;

class U1_L2_Activity_One {
    public static void main(String[] args) {
        final Scanner s = new Scanner(System.in);

        System.out.println("What is your favourite food?");
        final String content = s.nextLine();

        s.close();

        System.out.println("I like to eat " + content + " as well!");

    }
}
