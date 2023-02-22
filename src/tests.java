import java.util.Arrays;

public class tests {

  public static void main(String[] args) {
    int[][] matrix = new int[6][4];

    for (int i = 0; i < matrix.length - 1; i += 2) {
      for (int j = 0; j < matrix[i].length; j++) {
        matrix[i + 1][j] = j;
        matrix[i][matrix[i].length - j - 1] = i;
      }
    }

    str(matrix);
  }

  public static <T> void str(T[][] a) {
    for (T[] ts : a) {
      for (T t : ts) {
        System.out.print(t.toString() + " ");
      }

      System.out.println();
    }
  }

  public static void str(int[][] a) {
    for (int[] ts : a) {
      for (int t : ts) {
        System.out.print(t + " ");
      }

      System.out.println();
    }
  }

}