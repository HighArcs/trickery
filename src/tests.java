import java.util.Scanner;

public class tests {
  public static void main(String[] args) {
    String[] words = { "BEAUTIFY", "BENEVOLENCE", "BENIGN", "BEQUEST", "BERATED", "BEREFT", "BEWILDER" };
    int s = words.length - 1;
    for (int i = 0; i < words.length; i++) {
      if (words[s].length() > words[i].length()) {
        s = i;
      }
    }

    System.out.println(words[s]);
  }

  // this is such a good algorithm
  private static int square(int n) {
    // 0^2 -> 0
    if (n == 0) {
      return 0;
    }

    // (-n)^2 -> n^2
    if (n < 0) {
      n = -n;
    }

    // get n/2 as an int; will be used if even
    // n >> 1 is the same as n / 2 but faster
    final int x = n >> 1;

    // 2((n/2)^2); n << 2 is the same as 2n
    final int next = square(x) << 2;

    // check if n is odd if the binary value has a 1 bit
    // get this by using bitwise AND, if its even then n&1 will be 0

    if ((n & 1) != 0) {
      // DO NOT substitute x<<2 with n<<1;
      // they may be similar math wise
      // but the extra ending bit needs to stay
      return next + (x << 2) + 1;
    }

    // at this point, it can't *not* be even, since it ends otherwise
    return next;
  }
}
