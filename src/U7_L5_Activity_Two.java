import java.util.ArrayList;

public class U7_L5_Activity_Two {
    public static void selectSortReverse(ArrayList<Integer> vec) {
        for (int i = 0; i < vec.size() - 1; i++) {
            int min_idx = i;
            for (int j = i + 1; j < vec.size(); j++) {
                if (vec.get(j) > vec.get(min_idx)) {
                    min_idx = j;
                }
            }

            Integer temp = vec.get(min_idx);
            vec.set(min_idx, vec.get(i));
            vec.set(i, temp);
        }

        System.out.println(vec);
    }
}
