public class U6_L5_Activity_Two {
    public static int product(int[] arr) {
        int p = 1; // move out of fori
        for (int k : arr) { // +int
            p *= k;
        }

        return p;
    }
}
