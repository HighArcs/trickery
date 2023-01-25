1. Consider the following code:

```java
ArrayList<String> list = new ArrayList<String>();
list.add("to");
list.add("too");
list.add("two");
System.out.println(list.size());
```

What is printed when this code segment is executed?

> 3

2. Which of the following is NOT true about `ArrayList` objects? 

> `ArrayList` objects can hold primitive type data.

3. What is output by this code?

```java
ArrayList<String> laundry = new ArrayList<String>();

laundry.add("shirt");
laundry.add("sock");
laundry.add("sock");
laundry.add("sweater");
laundry.add("shorts");
laundry.remove(1);

System.out.println(laundry);
```

> [shirt, sock, sweater, shorts] 

4. Which is the correct import statement for the `ArrayList` class? 

> `import java.util.ArrayList;`

5. Consider the following code:

```java
ArrayList<String> shopping = new ArrayList<String>();
shopping.add("Bread");
shopping.add("Peanut butter");
shopping.add("Jelly");
System.out.println(shopping.get(1));
System.out.println(shopping);
```

What is printed when this code segment is executed?

> ```
> Peanut butter
> [Bread, Peanut butter, Jelly]
> ```

6. Suppose the following code is executed:

```java
ArrayList<String> words = new ArrayList<String>();
words.add("make");
words.add("unfold");
words.add("crustacean");
words.add("follow");
```

Which of the following method calls would change the element of words which is currently `"crustacean"` to be `"whittle"`?

> `words.set(2, "whittle")`

7. Consider the following code:

```java
ArrayList<String> words = new ArrayList<String>();
words.add("set");
words.add("interface");
words.add("limit");
words.add("testify");
words.add(2, "disagree");
System.out.println(words);
```

What would be printed when this code is executed?

> `[set, interface, disagree, limit, testify]`