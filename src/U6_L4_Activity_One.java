public class U6_L4_Activity_One {
    public static String insert(String[] words, String newWord, int place) {
        if (place < 0 || place >= words.length) {
            return "you need a valid number";
        }

        String prev = newWord;

        for (int i = place; i < words.length; i++) {
            String current = prev;
            prev = words[i];
            words[i] = current;
        }

        String output = "";

        for (int i = 0; i < words.length; i++) {
            output += words[i];
        }

        return output;
    }
}
