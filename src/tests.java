public class tests {
    public static void main(String[] args) {
        int x = 17;
        int y = 11;

        if (y < x || ((7 * x - 6 * y) % x) == 1) {
            System.out.print("yes");
        } else {
            System.out.print("no");
        }
    }
}
