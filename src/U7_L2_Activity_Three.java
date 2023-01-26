import java.util.ArrayList;

public class U7_L2_Activity_Three {
    public static ArrayList<Integer> getEvens(ArrayList<Integer> vec) {
        ArrayList<Integer> out = new ArrayList<Integer>();

        for (Integer i : vec) {
            if (i % 2 == 0) {
                out.add(i);
            }
        }

        return out;
    }
}
