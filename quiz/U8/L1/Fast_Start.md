1. Consider the following array declaration.

```java
int[] arr = {3, 6, 7, 9, 5};
```

What value is returned by `arr[4]`?

> 5

2. Which of the following code segments will print all values of an array of ints named arr with even indices?

> ```java
> for (int i = 0; i < arr.length; i+=2) {
>   System.out.println(arr[i] + " ");
> }
> ```

3. Consider the following loops:

I.
```java
for (int i = 0; i < list1.size(); i++) {
     System.out.println(list1.get(i));
}
```
II.
```java
for (int i = 0; i < list2.length; i++) {
     System.out.println(list2[i]);
}
```
III.
```java
for (int i : list3) {
     System.out.println(i);
}
```

Assuming that all three of these loops appear in the same method, and all three compile and execute without errors, and list1, list2 and list3 are all either arrays or ArrayLists, which of the following MUST be true? Select all that apply.

> list1 is an ArrayList
> 
> list2 is an array