public class U6_L5_Activity_Three {
    public static double avg(int[] arr) {
        int s = 0;
        for (int n : arr) {
            s += n; // ++ -> +=
        }

        return (double) s / (double) arr.length; // -(), move out of for loop

    }
}
