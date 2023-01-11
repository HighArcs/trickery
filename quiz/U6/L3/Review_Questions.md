1. Consider the following code:

What would be output when the following code is executed?

```java
String[] word = {"algorithm", "boolean", "int", "double"};
for (int i = 0; i < word.length / 2; i++) {
  word[i] = word[word.length - 1 - i];
  word[word.length - 1 - i] = word[i];
}

for (int i = 0; i < word.length; i++) {
  System.out.print(word[i] + " ");
}
```

> double int int double

2. Consider the following code:

```java
String[] words = { "BEAUTIFY", "BENEVOLENCE", "BENIGN", "BEQUEST", "BERATE","BEREFT", "BEWILDER" };
for (int i = 0; i < words.length; i++) {
  if (words[i].substring(0, 3).compareTo("BEN") != 0) {
    System.out.print(words[i] + " ");
  }
}
```

What is output?

> BEAUTIFY BEQUEST BERATE BEREFT BEWILDER

3. Consider the following code:

```java
String[] words = {"BEAUTIFY", "BENEVOLENCE", "BENIGN", "BEQUEST", "BERATE", "BEREFT", "BEWILDER"};
int s = 0;
for (int i = 0; i < words.length; i++) {
  if (words[s].length() < words[i].length()) {
    s = i;
  }
}

System.out.println(words[s]);
```

What is output?

> BENEVOLENCE

4. Consider the following code:

```java
String[] words = {"BEAUTIFY", "BENEVOLENCE", "BENIGN", "BEQUEST", "BERATED", "BEREFT", "BEWILDER"};
int s = words.length - 1;
for (int i = 0; i < words.length; i++) {
  if (words[s].length() > words[i].length()) {
    s = i;
  }
}

System.out.println(words[s]);
```

What is output?

> BENIGN

5. The following code is intended to count how many "hard" spelling words are assigned in a week. A hard spelling word is defined as having more than 5 letters.

Consider the following code:

```java
String[] list = /* initialized to the spelling words */;
int hard = 0;
for (int i = 0; i < list.length; i++) {
  if (/* missing code */) {
    hard++;
  }
}

System.out.println("Hard words: " + hard);
```

Which could be used to replace `/* missing code */` so that it works as intended?

> `list[i].length() > 5`

6. A student is having trouble remembering how to spell words that end in –ing. She needs a program to count how many ing words she has each week in her spelling list.

Consider the following code:

```java
String[] list = /* initialized to the spelling words */;
int ing = 0;
for (int i = 0; i < list.length; i++) {
  if (list[i].length() > 3) {
    if (/* missing code */) {
      ing++;
    }
  }
}

System.out.println("Number of ing words: " + ing);
```

Which of the following could be used to replace `/* missing code */` so that this works as intended?

> `list[i].substring(list[i].length() - 3, list[i].length()).equals("ing")`


