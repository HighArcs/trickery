import java.util.Scanner;

class U1_L6_Activity_Two {
    public static void main(String[] args) {

        final Scanner s = new Scanner(System.in);

        final int a = (int) Math.round(s.nextDouble());
        final int b = (int) Math.round(s.nextDouble());

        s.close();

        System.out.println("Answer: " + a + " + " + b + " = " + (a + b));
    }
}