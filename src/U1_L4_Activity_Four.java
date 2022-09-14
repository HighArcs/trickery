import java.util.Scanner;

class U1_L4_Activity_Four {
    public static void main(String[] args) {

        System.out.println("Enter a price:");

        Scanner s = new Scanner(System.in);

        double d = s.nextDouble();
        s.close();

        System.out.println("Change from $10: $" + (10 - d));
    }
}