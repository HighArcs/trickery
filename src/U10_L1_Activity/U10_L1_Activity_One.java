public class U10_L1_Activity_One {

    public static void printUpToEnd(int n) {
        if (n - 10 > 0) {
            printUpToEnd(n - 10);
        }
        System.out.print(n + " ");
    }

}
