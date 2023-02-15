public class U8_L1_Activity_Two {
    public static int[][] productTable(int rows, int columns) {
        int[][] buf = new int[rows][columns];

        for (int i = 0; i < buf.length; i++) {
            for (int j = 0; j < buf[i].length; j++) {
                buf[i][j] = i * j;
            }
        }

        return buf;
    }
}
