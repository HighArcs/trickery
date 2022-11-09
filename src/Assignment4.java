// this uses the zipper convention, which allows us to condense multiple for loops into one, and running the contents of both per iteration.
// we also use indexOf() != -1 and adding a single character to strings to check if something has been used before


import java.util.Scanner;

public class Assignment4 {
  public static void main(String[] args) {
    final Scanner s = new Scanner(System.in);
    
    System.out.println("Type the message to be shortened");
    // immediately set our string to its lowercase variant, we don't have to care about uppercase characters
    final String str = s.nextLine().toLowerCase();
    
    // close our scanner, stopping a memory leak
    s.close();
    
    String m1 = "";
    int vowels = 0;
    int repeat = 0;
    
    String m2 = "";
    String reached = " ";
    
    // iterate over the string ONE time.
    
    // this can be done due to the fact that both `for` loops take the exact same range
    // and that neither loop depends on the other or modifies eachother
    for (int i = 0; i < str.length(); i++) {
     final char c = str.charAt(i);
     
     // using loop labels; this is used to make sure we don't stop walking the string too early when we use continue.
     // the variable t is instantly exhausted, stopping the loop after exactly one iteration
     a1: for (int t = 0; t < 1; t++) {
       // always add the start of the string, regardless if it matches other criteria
       if (i == 0) {
         m1 += c;
         // `continue` here breaks us out of the current loop, in this case `a1`
         continue a1;
       }
       
       // get the previous character
       // we can guarantee this won't panic with the previous if check (if `i` was 0 then we would get the -1st character)
       final char p = str.charAt(i - 1);
       
       // if the character is preceded by a space, marking it the start of a word
       if (p == ' ') {
         m1 += c;
         continue a1;
       }
       
       // if the character is a vowel, excluding y
       if (c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u') {
         vowels++;
         continue a1;
       }
       
       // if the character is exactly the same as what was before it, meaning it had to have been repeated
       if (c == p) {
         repeat++;
         continue a1;
       }
       
       m1 += c;
     } // burn out the loop
     
     a2: for (int t = 0; t < 1; t++) {
       // check if the `reached` string contains the current character at all
       if (reached.indexOf(c + "") != -1) {
         // if not, skip this character
         continue a2;
       }
       
       // add this character to the reached string, letting it know we've seen it before
       reached += c;
       
       // start a counter, this is used in the output string
       int count = 0;
       // burn through the string again, could be optimized to use the outer loop but it becomes too complex to read it
       for (int j = 0; j < str.length(); j++) {
         // if the current character is the same as the one we're checking, add to the counter
         if (str.charAt(j) == c) {
           count++;
         }
       }
         
       // add the counter and the actual character to the final message
       m2 += count;
       m2 += c;
     }
    }
    
    System.out.println();
    System.out.println("Algorithm 1");
    System.out.println("Vowels removed: " + vowels);
    System.out.println("Repeats removed: " + repeat);
    System.out.println("Algorithm 1 message: " + m1);
    System.out.println("Algorithm 1 characters saved: " + (str.length() - m1.length()));
    
    System.out.println();
    System.out.println("Algorithm 2");
    System.out.println("Unique characters found: " + (reached.length() - 1)); // -1 removes the space from initial
    System.out.println("Algorithm 2 message: " + m2);
    System.out.println("Algorithm 2 characters saved: " + (str.length() - m2.length()));
  }
}
