import java.util.Scanner;

public class U2_L4_Activity_One {
    public static void main(String[] args) {

        Scanner scanner = new Scanner(System.in); // change `scanner` to `Scanner`

        // get first string
        System.out.println("Enter first string");
        String s1 = scanner.nextLine(); // add `String`

        // get second string
        System.out.println("Enter second string");
        String s2 = scanner.nextLine(); // change `string` to `String`

        // Get number of letters to use from each string
        System.out.println("Enter number of letters from each word");
        int n = scanner.nextInt(); // change `num` to `int`

        // print last n letters of first string combined with first n letters of s2
        System.out.println(s1.substring(s1.length() - n) + s2.substring(0, n));
        // change the `+` between
        // `s1.length` and `n` to a -

        // remove second substring param and add `n` to the second set

        // close scanner
        scanner.close();
    }
}
