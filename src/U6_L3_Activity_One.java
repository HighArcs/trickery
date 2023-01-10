public class U6_L3_Activity_One {
    public static String findShortest(String[] words) {
        String output = null;

        for (int i = 0; i < words.length; i++) {
            if (output == null || words[i].length() < output.length()) {
                output = words[i];
            }
        }

        return output;
    }
}
