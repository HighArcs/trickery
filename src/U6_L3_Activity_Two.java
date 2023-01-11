public class U6_L3_Activity_Two {
    public static void removeVowels(String[] words) {
        for (int i = 0; i < words.length; i++) {
            for (int j = 0; j < words[i].length(); j++) {
                String slice = words[i].substring(j, j + 1);

                if (!(slice.equals("a") || slice.equals("e") || slice.equals("i") || slice.equals("o")
                        || slice.equals("u"))) {
                    System.out.print(slice);
                }
            }

            System.out.print("\n");
        }
    }
}
