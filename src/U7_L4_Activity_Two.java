import java.util.ArrayList;

public class U7_L4_Activity_Two {
    public static int searchSecond(ArrayList<String> vec, String item) {
        for (int i = 0; i < vec.size(); i++) {
            String word = vec.get(i);

            if (word.equals(item)) {
                // skip i
                for (int j = i + 1; j < vec.size(); j++) {
                    if (vec.get(j).equals(item)) {
                        return j;
                    }
                }
            }
        }

        return -1;
    }
}
