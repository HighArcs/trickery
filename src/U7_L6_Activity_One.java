public class U7_L6_Activity_One {
    public static void sortAndPrintReverse(String[] arr) {
        for (int i = 1; i < arr.length; i++) {
            String to_insert = arr[i];
            int j;

            for (j = i - 1; j >= 0; j--) {
                if (to_insert.compareTo(arr[j]) >= 0) {
                    arr[j + 1] = arr[j];
                } else {
                    break;
                }
            }

            arr[j + 1] = to_insert;
            System.out.println(toString(arr));
        }
    }

    public static String toString(String[] arr) {
        return String.join(" ", arr);
    }
}
