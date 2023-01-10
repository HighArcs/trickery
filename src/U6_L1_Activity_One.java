import java.util.Scanner;

public class U6_L1_Activity_One {
    public static void main(String[] args) {

        Scanner scan = new Scanner(System.in);

        double[] arr = new double[3]; // new int(3) -> new double[3]
        arr[0] = scan.nextDouble(); // change indices to zbi
        arr[1] = scan.nextDouble();
        arr[2] = scan.nextDouble();

        System.out.println("Contents: " + arr[0] + " " + arr[1] + " " + arr[2]); // arr(n) -> arr[n - 1]
        System.out.println("Sum: " + (arr[0] + arr[1] + arr[2])); // add parens

        scan.close();
    }
}
