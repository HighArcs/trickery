import java.util.Scanner;

class U1_L3_Activity_Three {
    public static void main(String[] args) {
        final Scanner s = new Scanner(System.in);

        System.out.println("Java is an object-oriented programming language, true or false?");
        final boolean u = s.nextBoolean();

        System.out.println("There are only 2 possible values which can be held by a boolean variable, true or false?");
        final boolean p = s.nextBoolean();

        s.close();
        
        System.out.println("Question 1 - Your answer: " + u + ". Correct answer: true");
        System.out.println("Question 2 - Your answer: " + p + ". Correct answer: true");

    }
}
