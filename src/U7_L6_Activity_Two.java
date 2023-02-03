import java.util.ArrayList;

public class U7_L6_Activity_Two {
    public static int insertSort(ArrayList<Integer> arr) {
        int count = 0;
        for (int i = 1; i < arr.size(); i++) {
            Integer val = arr.get(i);
            int j;

            for (j = i - 1; j >= 0; j--) {
                count++;
                if (arr.get(j) > val) {
                    arr.set(j + 1, arr.get(j));
                } else {
                    break;
                }
            }

            arr.set(j + 1, val);
        }

        return count;
    }
}
