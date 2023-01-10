public class U6_L2_Activity_Three {
    public static int numDivisible(int num, int[] vec) {
        int count = 0;
        for (int i = 0; i < vec.length; i++) {
            if (vec[i] % num == 0) {
                count++;
            }
        }

        return count;
    }
}
