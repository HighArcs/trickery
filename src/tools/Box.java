package tools;


public class Box<T> {
    private T value;

    public Box(T x) {
        this.value = x;
    }

    public static final <T> Box<T> uninitialized() {
        return new Box<>(null);
    }

    public final Box<T> write(T x) {
        this.value = x;
        return this;
    }

    public final T read() {
        return this.value;
    }

    public final Box<Box<T>> ref() {
        return new Box<Box<T>>(this);
    }

    public final Box<T> drop() {
        this.value = null;
        return this;
    }

    public final boolean filled() {
        return this.value != null;
    }

    public final boolean empty() {
        return !this.filled();
    }
}