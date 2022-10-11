import java.util.Scanner;

public class tests {
    public static void main(String[] args) {
        final Scanner scan = new Scanner(System.in);

        int base = 2;
        int exponent = scan.nextInt();
        int answer = (int) Math.pow(base, exponent);
        if (answer <= 256) {
            System.out.println("exponent is 8 or less");
        }

        scan.close();

    }
}
