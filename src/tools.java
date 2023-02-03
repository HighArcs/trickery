public class tools {
    public static <T> String to_string(T[] values) {
        String mut = "{ ";
        for (T t : values) {
            mut += t.toString();
            mut += ", ";
        }

        mut = mut.replaceAll(", $", " ");

        mut += "}";

        return mut;
    }
}
