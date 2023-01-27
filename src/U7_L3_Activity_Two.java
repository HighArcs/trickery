import java.util.ArrayList;

public class U7_L3_Activity_Two {
    public static void printStatistics(ArrayList<Integer> vec) {
        int sum = 0;
        int mode = vec.get(0);
        boolean has_mode = false;
        int max_count = 0;

        for (int i = 0; i < vec.size(); i++) {
            sum += vec.get(i);
            int count = 1;
            for (int j = i + 1; j < vec.size(); j++) {
                if (vec.get(i).equals(vec.get(j))) {
                    count++;
                }
            }

            if (count > max_count) {
                mode = vec.get(i);
                has_mode = true;
                max_count = count;
            }

            else if (count == max_count) {
                has_mode = false;
            }
        }

        System.out.println("Sum: " + sum);
        System.out.println("Average: " + ((double) sum / vec.size()));
        System.out.println("Mode: " + (has_mode ? mode : "no single mode"));
    }
}
