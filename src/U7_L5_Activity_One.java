public class U7_L5_Activity_One {
    public static void sortAndPrintReverse(String[] vec) {
        // int n = ;
        for (int i = 0; i < vec.length - 1; i++) {
            int min_idx = i;
            for (int j = i + 1; j < vec.length; j++) {
                if (vec[j].compareTo(vec[min_idx]) > 0) {
                    min_idx = j;
                }
            }

            String temp = vec[min_idx];
            vec[min_idx] = vec[i];
            vec[i] = temp;
        }


        for (String string : vec) {
            System.out.print(string + " ");    
        }
    }

}
