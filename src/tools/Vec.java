package tools;

public class Vec<T> {
    private Box<T>[] alloc = Unsafe.array(0);

    public Vec() {
    }

    public static <T> Vec<T> withCapacity(int capacity) {
        final Vec<T> vec = new Vec<T>();
        vec.alloc = Unsafe.array(capacity);
        return vec;
    }

    public final int capacity() {
        return this.alloc.length;
    }

    private Vec<T> extendCapacity() {
        this.alloc = Unsafe.setLength(this.alloc, this.capacity() * 2);
        return this;
    }

    public final Vec<T> reserve(int additional) {
        while (this.capacity() < additional) {
            this.extendCapacity();
        }

        return this;
    }

    public final Vec<T> reserveExact(int additional) {
        if (this.capacity() < additional) {
            this.alloc = Unsafe.setLength(this.alloc, additional);
        }

        return this;
    }

    private final boolean hasNoneTail() {
        final Box<T> last = this.alloc[this.alloc.length - 1];

        return last == null;
    }

    public final Vec<T> shrinkToFit() {
        while (this.hasNoneTail()) {
            this.alloc = Unsafe.pop(this.alloc);
        }

        return this;
    }

    public final Vec<T> shrinkTo(int to) {
        while (this.capacity() > to && this.hasNoneTail()) {
            this.alloc = Unsafe.pop(this.alloc);
        }

        return this;
    }

    public final int len() {
        int c = 0;
        int i = 0;
        while (i < this.alloc.length && this.alloc[i++].filled()) {
            c++;
        }

        return c;
    }

    public final Vec<T> truncate(int len) {
        while (this.len() > len) {
            this.alloc[this.len() - 1] = null;
        }

        return this;
    }

    // asSlice

    public final Box<T> get(int index) {
        if (index < 0 || index > this.len()) {
            return Box.uninitialized();
        }

        return this.alloc[index];
    }

    public final T getUnchecked(int index) {
        return this.alloc[index].read();
    }

    private final Box<T> last() {
        return this.get(this.len() - 1);
    }

    private final void oob(int value) throws Exception {
        throw new Exception("index " + value + " does not fit in vec["+this.len()+"; " + this.capacity() + "]");
    }

}
