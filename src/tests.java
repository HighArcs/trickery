public class tests {
    public static void main(String[] args) {
        int x = 5;
        if (!(x > 5 || x <= 2)) {
            System.out.println("in range 1");
        } else {
            System.out.println("not in range 1");
        }

        if (!(x >= 4 && x < 7)) {
            System.out.println("in range 2");
        } else {
            System.out.println("not in range 2");
        }
    }
}
