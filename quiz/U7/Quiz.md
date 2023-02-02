1. What goes in between the < > when declaring a new ArrayList?

> A class data type

2. Consider the following code segment.

```java
ArrayList<String> stuff = new ArrayList<String>();
stuff.add("Z");
stuff.add("f");
stuff.add(2, "W");
stuff.remove(1);
stuff.add("x");
System.out.println(stuff);
```

What is printed as a result of running this code segment?

> `[Z, W, x]`



Questions 3 - 5 refer to the following code:

```java
public class Whatchamacallit {
  private double price;
  private String title;

  public Whatchamacallit() {
    this(0, "none");
  }

  public Whatchamacallit(double p, String t) {
    price = 0;
    if (p > 0) {
      price = p;
    }

    title = t;
  }

  public String toString() {
    return title + " costs $" + price;
  }
}
```
The following code segment appears in another class:

```java
ArrayList<Whatchamacallit> list = new ArrayList<Whatchamacallit>();

list.add(new Whatchamacallit());
list.add(new Whatchamacallit(3.5, "book"));
list.add(new Whatchamacallit(-17, "CD"));
list.add(new Whatchamacallit(18.95, "sweater"));
list.add(new Whatchamacallit(5, "notebook"));

/* Missing Code */
```

3. Suppose the following line is used to replace `/* Missing Code */`.

```java
System.out.println(list.get(0));
```

What is printed as a result of executing the code segment?

> none costs $0.0

1. Suppose the following lines are used to replace `/* Missing Code */`.

```java
list.remove(1);
System.out.println(list.get(1));
```

What is printed as a result of executing the code segment?

> CD costs $0.0
6. Consider the following code segment:

```java
ArrayList<Light> bulbs = new ArrayList<Light>();
bulbs.add(new Light());
bulbs.remove(0);
bulbs.add(new Light());
Light b = new Light();
bulbs.add(1, b);
bulbs.add(new Light());
bulbs.remove(0);
bulbs.add(new Light());
bulbs.remove(2);
bulbs.add(new Light());
bulbs.add(1, new Light());
```

After running the code, what is the size of bulbs?

> 4

7. Consider the following method that is intended to test if all the Strings in the ArrayList start with an uppercase letter:

```java
public static boolean capitalized(ArrayList<String> a) {
    /* Missing Code */
}
```


Which of the following could replace `/* Missing Code */` so that the method works as intended? (You may assume all strings in the ArrayList begin with a letter character.)

I.
```java
for (String s : a) {
  if (!s.toUpperCase().substring(0, 1).equals(s.substring(0, 1))) 
  {
    return true;    
  }
}
return false;
```

II. 	
```java
for (String s : a) {
  if (!s.toUpperCase().substring(0, 1).equals(s.substring(0, 1))) {
    return false;    
  }
}
return true;
```

III. 	
```java
int flag = 1;
for (String s : a) {
  if (!s.toUpperCase().substring(0, 1).equals(s.substring(0, 1))) {
    flag = 0;    
  }
}

return (flag == 1);
```
> II and III

8. Consider the following method intended to modify the parameter names by removing all instances of the String n.

```java
public static void removeNames(ArrayList<String> names, String n) {
  for (/* Missing Code */) {
    if (names.get(i).equals(n)) {
      names.remove(i);
    }
  }
}
```

Which of the following could correctly replace `/* Missing Code */` so that removeNames works as intended?

> `int i = 0; i < names.size(); i++`

9. Consider the following method.

```java
public ArrayList<Integer> sequence(int n) {
  ArrayList<Integer> list = new ArrayList<Integer>();

  for (int k = 0; k < n; k++) {
    list.add(new Integer(n + 2 * k));
  }

  return list;
}
```

Which of the following is printed as a result of executing the following statement?

```java
System.out.println(sequence(5));
```

> `[5, 7, 9, 11, 13]`

10. Consider the following method: 

```java
public static void mystery(ArrayList<String> words) {
  int k = 0;
  int newSize = 3 * words.size();

  while (words.size() < newSize && words.size() > 0) {
    words.add(words.get(k));
    k++;
  }
}
```

Which of the following describes what the method `mystery()` does to the ArrayList words?

> The ArrayList words is extended to three times its size by repeating the entire contents of the list three times in sequence. 