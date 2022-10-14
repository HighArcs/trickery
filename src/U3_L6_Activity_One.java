import java.util.Scanner;

public class U3_L6_Activity_One {
    public static void main(String[] args) {
        final Scanner s = new Scanner(System.in);
        System.out.println("Enter a number in the fifties");
        int num = s.nextInt();

        s.close();

        if (num < 50 || num >= 60) { // change && to || and > to >=
            System.out.println("That's not in the fifties!");
            num = 55;
        }
        System.out.println("Your number is " + num);
    }
}
