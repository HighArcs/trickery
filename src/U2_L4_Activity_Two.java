import java.util.Scanner;

public class U2_L4_Activity_Two {
    public static void main(String[] args) {
        Scanner scan = new Scanner(System.in); // add `Scanner`
        String str1 = scan.nextLine();
        String str2 = new String(str1); // add `new`
        str1 = str1.toUpperCase(); // edit syntax
        str2 = str2.substring(0, 1).toUpperCase() + str2.substring(1);
        System.out.println(str2); // remove quotes
        System.out.println(str1); // remove quotes

        scan.close();
    }
}