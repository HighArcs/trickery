import java.util.Scanner;

class U1_L6_Activity_Three {
    public static void main(String[] args) {

        final Scanner s = new Scanner(System.in);

        final double d = s.nextDouble();

        s.close();

        final int a = (int) (d * 10 % 10);
        final int b = (int) (d * 100 % 10);
        final int c = (int) (d * 1000 % 10);

        System.out.println(String.format("Answer: %d %d %d", a, b, c));
        System.out.println("Answer: " + a + " " + b + " " + c);
    }
}