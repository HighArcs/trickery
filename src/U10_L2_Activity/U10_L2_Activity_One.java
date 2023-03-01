public class U10_L2_Activity_One {
    public static String stringReverse(String word) {
        if (word.length() <= 1) {
            return word;
        }

        return stringReverse(word.substring(1)) + word.substring(0, 1);
    }
}
