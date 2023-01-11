public class U6_L2_Activity_Two {
    public static int numDivisibleBy3(int[] vec) {
        int count = 0;
        for (int i = 0; i < vec.length; i++) {
            if (vec[i] % 3 == 0) {
                count++;
            }
        }

        return count;
    }
}
