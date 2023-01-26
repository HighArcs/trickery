import java.util.ArrayList;

public class U7_L2_Activity_Two {
    public static Integer highestNum(ArrayList<Integer> vec) {
        Integer max = Integer.MIN_VALUE;
        for (Integer i : vec) {
            if (i > max) {
                max = i;
            }
        }

        return max;
    }
}
