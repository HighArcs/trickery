import java.util.Scanner;

class U1_L6_Activity_One {
    public static void main(String[] args) {
        final Scanner s = new Scanner(System.in);

        final int n = s.nextInt();
        final int d = s.nextInt();

        s.close();

        System.out.println("The decimal value is: " + (double) n / d);
    }
}
