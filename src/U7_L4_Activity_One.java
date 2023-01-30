import java.util.ArrayList;

public class U7_L4_Activity_One {
    public static int countSecondInitial(ArrayList<String> list, String letter) {
        int count = 0;
        for (String word : list) {
            if (word.substring(1, 2).equalsIgnoreCase(letter)) {
                count++;
            }
        }

        return count;
    }
}
