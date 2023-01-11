1. Create an array of every number between 1 and 5 inclusive.

> `int[] anArray = { 1, 2, 3, 4, 5 };`

2. What property of an array do you use to find how long an array is? 

> `length`

3. Consider the following code:

```java
String w = "onomatopoeia";
for (int i = 0; i < w.length(); i++) {
  System.out.print(w.substring(i, i + 1) + "  ");
  if (i % 3 == 2) {
    System.out.println();
  }
}
```

What is output?

> o  n  o  
> 
> m  a  t  
> 
> o  p  o  
> 
> e  i  a  

