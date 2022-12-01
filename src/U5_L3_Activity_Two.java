import shapes.*;
import testing.Math;

public class U5_L3_Activity_Two {
    public static void randomize(Rectangle rec) {
        // 0..1 -> 0..10 -> int 10..21 -> 10..=20
        int l = (int) (Math.random() * 10) + 11;

        // ensure it is even
        if (l % 2 == 1) {
            l--;
        }

        rec.setLength(l);

        // 0..1 -> 0..7 -> int 7..14 -> 7..=13
        int w = (int) (Math.random() * 7) + 7;

        if (w % 2 == 0) {
            w--;
        }

        rec.setWidth(w);
    }
}
