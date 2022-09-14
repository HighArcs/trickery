import java.util.Scanner;

class U1_L5_Activity_Two {
    public static void main(String[] args) {
        Scanner s = new Scanner(System.in);

        int i = s.nextInt();
        s.close();

        int a = i / 1000 % 10;
        int b = i / 100 % 10;
        int c = i / 10 % 10;
        int d = i % 10;

        System.out.println(d);
        System.out.println(c);
        System.out.println(b);
        System.out.println(a);

        return;
    }
}
