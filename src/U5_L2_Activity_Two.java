public class U5_L2_Activity_Two {
    public static void reverser(String str) {
        for (int i = str.length() - 1; i >= 0; i--) {
            System.out.print(str.substring(i, i + 1));
        }
    }
}
