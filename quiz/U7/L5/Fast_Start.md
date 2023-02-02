1. Assuming all variables are declared correctly, which of the following code segments correctly swaps the values x and y?

I. 	
```java
x = y;
y = x;
```

II. 	
```java
int tmp = x;
y = x;
x = tmp;
```

III. 	

```java
int tmp = x;
x = y;
y = tmp;
```

> III only

2. Consider the following code:

```java
public boolean mystery(int[] a) {
  boolean flag = true;
  for (int i = 1; i < a.length; i++) {
    if (a[i] < a[i - 1]) {
      flag = false;
      break;
    }
  }

  return flag;
}
```

What does `mystery(int[])` do?

> Tests if the array is in ascending order. 

