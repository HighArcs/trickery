import java.util.Scanner;

public class U4_L1_5_Activity_One {
    public static void main(String[] args) {
        final Scanner s = new Scanner(System.in);

        int num = s.nextInt();
        s.close();

        int fac = 1;
        while (fac <= 5) { // < -> <=
            // swap lines
            System.out.println(num * fac);
            fac++;
        }

    }
}