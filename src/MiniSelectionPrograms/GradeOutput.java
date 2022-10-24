package MiniSelectionPrograms;

import java.util.Scanner;

public class GradeOutput {
    public static void main(String[] args) {
        final Scanner s = new Scanner(System.in);

        System.out.println("Enter your grade as an integer:");
        final int grade = s.nextInt();

        s.close();

        if (grade < 0 || grade >= 100) {
            System.out.println("Bad grade!");
            return;
        }

        if (grade >= 98) {
            System.out.println("A+");
        } else {
            
        }
    }
}