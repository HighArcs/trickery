import java.util.ArrayList;
import java.util.Scanner;

public class U7_L1_Activity_One {
    public static void main(String[] args) {
        final Scanner scanner = new Scanner(System.in);

        final ArrayList<String> vec = new ArrayList<String>();
        System.out.println("Please enter words, enter STOP to stop the loop.");
        while (true) {
            final String line = scanner.nextLine();
            if (line.equals("STOP")) {
                break;
            }

            vec.add(line);
        }

        scanner.close();

        System.out.println(vec.size());
        System.out.println(vec);

        if (vec.size() > 2) {
            final String temp = vec.get(0);
            vec.set(0, vec.get(vec.size() - 1));
            vec.set(vec.size() - 1, temp);
            vec.remove(0);
        }

        System.out.println(vec);

    }
}
