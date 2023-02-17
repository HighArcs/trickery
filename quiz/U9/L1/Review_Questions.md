### 1. The term extends two classes to.

> Create a parent/child relationship

**Note**: Questions 2-4 refer to the following class headers.

```java
public class G { /* class code */ }
public class J extends G { /* class code */ }
public class P extends G { /* class code */ }
public class Q extends J { /* class code */ }
public class T extends J { /* class code */ }
```

### 2. True or false: a `T` is-a `G`?

> True

### 3. True or false: a `Q` is-a `P`?

> False

### 4. True or false: a `J` is-a `T`?

> False

### 5. Which of the following is false of a superclass?

> A superclass should contain methods that will be used only by the superclass

### 6. Consider the following class definitions.

```java
public class Nuts {
    private int Amt;
    
    public Nuts() {
        Amt = 0;
    }

    public Nuts(int x) {
        Amt = x;
    }
}

public class RoastedNuts extends Nuts {
    public RoastedNuts(int x) {
        super(x);
    }
}
```

Which of the following will not compile?

> `RoastedNuts RoastedMacadamia = new Nuts();` 

### 7. Consider the following class declaration.

```java
public class Subtracter {
    private int value;

    public Subtracter(int n) {
        value = n;
    }

    public void decrease(int less) {
        value = value - less;
    }

    public int getValue() {
        return value;
    }
}
```

The following code appears in another class:

```java
Subtracter a = new Subtracter(100);
Subtracter b = new Subtracter(25);
Subtracter c = b;

c.decrease(50);

System.out.println(a.getValue() + " " + b.getValue() + " " + c.getValue());
```

What is printed?

> 100 -25 -25