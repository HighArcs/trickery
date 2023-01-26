import java.util.ArrayList;
import java.util.Scanner;

public class U7_L2_Activity_One {
    public static void main(String[] args) {
        final Scanner s = new Scanner(System.in);

        final ArrayList<String> list = new ArrayList<String>();
        while (true) {
            final String line = s.nextLine();

            if (line.equals("STOP")) { break; }
            list.add(line);
        }

        System.out.println(list);

        for (int i = list.size() - 1; i >= 0; i--) {
            final String first = list.get(i);
            final String last = list.get(list.size() - i - 1);
            System.out.print(first);
            System.out.println(last);
        }

        s.close();
    }
}
