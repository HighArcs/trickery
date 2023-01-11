public class U6_L3_Activity_Three {
    public static void printUn(String[] vec) {
        for (int i = 0; i < vec.length; i++) {
            if (vec[i].length() >= 2 && vec[i].substring(0, 2).equals("un")) {
                System.out.println(vec[i]);
            }
        }
    }
}
