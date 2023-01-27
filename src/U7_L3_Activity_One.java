import java.util.ArrayList;

public class U7_L3_Activity_One {
    public static void shiftLeft(ArrayList<String> vec) {
        vec.add(vec.size() - 1, vec.remove(0));
    }
}
