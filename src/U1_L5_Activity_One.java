import java.util.Scanner;

class U1_L5_Activity_One {
    public static void main(String[] args) {
        final Scanner s = new Scanner(System.in);

        final int i = s.nextInt();
        s.close();

        final int a = i / 100;
        final int b = i / 10 % 10;
        final int c = i % 10;

        System.out.println(a);
        System.out.println(b);
        System.out.println(c);

    }
}
