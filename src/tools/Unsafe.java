package tools;

public class Unsafe {
    public static final <T> T[] array(int len) {
        return (T[]) new Object[len];
    }

    public static final <T, U> U cast(T t) {
        return (U) t;
    }

    public static final <T> T[] setLength(T[] array, int len) {
        final T[] alloc = Unsafe.array(len);

        for (int i = 0; i < alloc.length; i++) {
            if (i > array.length) {
                alloc[i] = null;
            } else {
                // move?
                alloc[i] = array[i];
            }
        }

        return alloc;
    }

    public static final <T> T[] pop(T[] t, int n) {
        return Unsafe.setLength(t, t.length - n);
    }

    public static final <T> T[] pop(T[] t) {
        return Unsafe.pop(t, 1);
    }

    public static final <T> T[] extend(T[] t, int n) {
        return Unsafe.setLength(t, t.length + n);
    }

    public static final <T> T[] extend(T[] t) {
        return Unsafe.pop(t, 1);
    }

    
}
