public class tests {
    public static void main(String[] args) {
        String str = "thinking";
        String start = str.substring(0, 2);
        String end = str.substring(str.length() - 3);

        if (start.equals("th") && end.equals("ing")) {
            System.out.println("Test 1 passed");
        }
        if (!str.equals("thing")) {
            System.out.println("Test 2 passed");
        }
    }
}
