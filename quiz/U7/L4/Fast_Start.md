1. Which of the following MUST be features of any algorithm? 

> Finishes in finite time
> 
> Consists of a set of logical operations

2. Which of the following will change the value of the element currently at index 3 of the ArrayList named list to "new value"?

> `list.set(3, "new value");`

3. Consider the following method.

```java
public static void repeatsAway(ArrayList<Integer> list) {
  for (int i = list.size() - 1; i > 0; i--) {
    if (list.get(i).equals(list.get(i-1))) {
      list.remove(i);
    }
  }
}
```

The following code appears in the main method of the same class. What is printed when this code is executed?

```java
ArrayList<Integer> a = new ArrayList<Integer> ();
a.add(2);
a.add(2);
a.add(7);
a.add(5);
a.add(7);
a.add(7);
a.add(7);
repeatsAway(a);
System.out.println(a);
```

> `[2, 7, 5, 7]`