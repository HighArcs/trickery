import java.util.Scanner;

public class U3_L7_Activity_One {
    public static void main(String[] args) {
        final Scanner s = new Scanner(System.in);
        
        System.out.println("Enter password:");
        final String input = s.nextLine();
        
        s.close();

        if (input.equals("bulbasaur")) {
            System.out.println("Access granted!");
        } else {
            System.out.println("Access denied!");
        }
    }
}
