import java.util.Scanner;

class U1_L5_Activity_Two {
    public static void main(String[] args) {
        final Scanner s = new Scanner(System.in);

        final int i = s.nextInt();
        s.close();

        final int a = i / 1000 % 10;
        final int b = i / 100 % 10;
        final int c = i / 10 % 10;
        final int d = i % 10;

        System.out.println(d);
        System.out.println(c);
        System.out.println(b);
        System.out.println(a);
    }
}
