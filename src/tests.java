public class tests {
    public static void main(String[] args) {
        int x = 11;
        int y = 11;

        if (x != y) {
            System.out.print("one");
        } else if (x > y) {
            System.out.print("two");
        } else if (y < x) {
            System.out.print("three");
        } else if (y >= x) {
            System.out.print("four");
        } else {
            System.out.print("five");
        }
    }
}
