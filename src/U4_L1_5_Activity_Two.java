import java.util.Scanner;

public class U4_L1_5_Activity_Two {
    public static void main(String[] args) {
        final Scanner s = new Scanner(System.in);
        
        int n = s.nextInt();
        
        s.close();

        int sum = 0;
        while (n > 0) {
            sum += n--;
        }

        System.out.println(sum);
    }
}
