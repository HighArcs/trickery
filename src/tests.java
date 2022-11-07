import java.util.Scanner;

public class tests {
    public static void main(String[] args) {
        String str = "an anaconda and an ant";
        int count = 0;
        for (int i = 0; i < str.length() - 1; i++) {
            if (str.substring(i, i + 2).equals("an")) {
                count++;
            }
        }

        System.out.println(count);
    }
}
