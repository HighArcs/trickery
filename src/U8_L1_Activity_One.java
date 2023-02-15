public class U8_L1_Activity_One {
    public static int sumOfDiag(int[][] arr) {
        int sum = 0;
        // for (int i = 0; i < arr.length; i++) {
        //     for (int j = 0; j < arr[i].length; j++) {
        //         if (i == j) {
        //             sum += arr[i][j];
        //         }
        //     }
        // }

        for (int i = 0; i < arr.length && i < arr[0].length; i++) {
            sum += arr[i][i];
        }

        return sum;
    }
}
