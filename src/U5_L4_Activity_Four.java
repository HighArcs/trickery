public class U5_L4_Activity_Four {
    public static int substringCount(String content, String sub) {
        int last = 0;
        int count = 0;

        while (last != -1) {
            last = content.indexOf(sub, last);

            if (last != -1) {
                count++;
                last += sub.length();
            }
        }

        return count;
    }
}
