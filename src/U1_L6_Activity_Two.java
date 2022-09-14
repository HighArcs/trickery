/* Lesson 4 Coding Activity Question 1 */

import java.util.Scanner;

class U1_L6_Activity_Two {
    public static void main(String[] args) {

        Scanner s = new Scanner(System.in);

        final int a = U1_L6_Activity_Two.round(s.nextDouble());
        final int b = U1_L6_Activity_Two.round(s.nextDouble());

        s.close();

        // System.out.println(String.format("Answer %d + %d = %d", a, b, a + b));
        System.out.println("Answer: " + a + " + " + b + " = " + (a + b));
    }

    public static int round(double value) {
        final int i = (int) value | 0;
        final double dx = value - (double) i;
        if (dx >= 0.5) {
            return i + 1;
        }

        return i;
    }
}