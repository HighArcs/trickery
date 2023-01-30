1. Which of the following best describes what the method mystery defined below does?

```java
public static void mystery(ArrayList<String> list) {
  for (int i = list.size() - 1; i > 0; i--) {
    if (list.get(i).equals(list.get(i - 1))) {
      list.add(i, "");
    }
  }
}
```
> This method adds a blank String between any two consecutive elements which are the same. 

2. Which of the following code segments will remove every String which is less than 3 characters long in the ArrayList of Strings, aList?

> ```java
> for (int i = 0; i < aList.size(); i++) {
>  if (aList.get(i).length() < 3) {
>    aList.remove(i);
>  }
> }
> ```

3. Consider the following two methods:

```java
public static void method1(ArrayList<String> myList) {
  for (int i = myList.size() - 1; i >= 0; i--) {
    if (myList.get(i).contains("th")) {
      myList.add(myList.get(i));
    }
  }
}
```
```java
public static void method2(ArrayList<String> myList) {
  for (int i = myList.size() - 1; i >= 0; i--) {
    if (myList.get(i).contains("th")) {
      myList.add(myList.remove(i));
    }
  }
}
```

Which of the following best describes the difference between method1 and method2?

> method1 adds a copy of every String containing "th" to the end of the list while method 2 moves every element containing "th" to the end of myList. 

4. Which of the following method definitions is equivalent to the one below (i.e. returns the same value in every possible case)?

```java
public static int numMatching(ArrayList<Object> list, Object thing){
  int count = 0;
  for (int i = 0; i < list.size(); i++)
  {
    if (list.get(i).equals(thing))
    {
      count++;
    }
  }
  return count;
}
```

> dc