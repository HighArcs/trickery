import shapes.*;
import testing.Math;

public class U5_L3_Activity_Two {
    public static void randomize(Rectangle rec) {
        rec.setLength(2 * (5 * Math.random() + 5));
        rec.setWidth((7 * Math.random() + 7));
    }
}
