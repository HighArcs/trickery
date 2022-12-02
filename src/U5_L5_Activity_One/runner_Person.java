// @ignore Java(536871240)
// package U5_L5_Activity_One;

import java.util.Scanner;

public class runner_Person {

    public static void main(String[] args) {

        final Scanner s = new Scanner(System.in);

        System.out.println("Enter the person's first name:");
        final String firstName = s.nextLine();

        System.out.println("Enter the person's last name:");
        final String lastName = s.nextLine();

        System.out.println("Enter the person's age:");
        final int age = s.nextInt();
        s.nextLine();

        System.out.println("Enter the person's social security number:");
        final String ssn = s.nextLine();

        System.out.println();
        final Person person = new Person(firstName, lastName, age, ssn);
        System.out.println(person);
    }
}
