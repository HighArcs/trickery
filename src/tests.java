import java.util.Scanner;

public class tests {
    public static void main(String[] args) {
        int c = 0;
        String str = "rebellion";
        int i = 0;
        int j = str.length() - 1;
        String result = "";
        while (i < j) {
            result = str.substring(i, i + 1) + result + str.substring(j, j + 1);
            System.out.println("ran " + ++c);
            i++;
            j--;
        }
        System.out.println(result);
    }
}
