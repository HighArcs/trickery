package tools;

import java.util.concurrent.Callable;

interface IsSomeAnd<T> {
    abstract boolean isSomeAnd(T t);
}

interface UnwrapOrElse<T> {
    abstract boolean isSomeAnd(T t);
}

public class Option<T> {
    private T value = null;

    public Option(T value) {
        this.value = value;
    }

    public final boolean isSome() {
        return this.value != null;
    }

    public final boolean isSomeAnd(IsSomeAnd<T> f) {
        return this.isSome() && f.isSomeAnd(this.value);
    }

    public final boolean isNone() {
        return this.value == null;
    }

    // asRef asMut asPinRef asPinMut asSlice asMutSlice

    public final T expect(String msg) throws Exception {
        if (this.isNone()) {
            throw new Exception(msg);
        }

        return this.value;
    }

    public final T unwrap() throws Exception {
        return this.expect("called Option::unwrap() on `None` value");
    }

    public final T unwrapOr(T def) {
        if (this.isNone()) {
            return def;
        }

        return this.value;
    }

    // unwrapOrElse, unwrapOrDefault

    public final T unwrapUnchecked() {
        return this.value;
    }

    public final static <T> Option<T> Some(T value) {
        return new Option<T>(value);
    }

    public final static <T> Option<T> None() {
        return new Option<T>(null);
    }
}
