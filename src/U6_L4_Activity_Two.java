public class U6_L4_Activity_Two {
    public static void swap(int[] vec, int i, int j) {
        if (!(i < 0 || j < 0 || i > vec.length || j > vec.length)) {
            int t = vec[i];
            vec[i] = vec[j];
            vec[j] = t;
        }
    }

    public static void allReverseSwap(int[] a) {
        for (int i = 0; i < a.length / 2; i++) {
            int t = a[i];
            a[i] = a[a.length - i - 1];
            a[a.length - i - 1] = t;

        }
    }
}
