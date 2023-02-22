use crate::tools::Scanner;

pub fn activity_one() {
    let scanner = Scanner::new(); // `scanner` -> `Scanner`

    // get first string
    writeln!(f, "Enter first string");
    let s1 = scanner.next_line();

    // get second string
    writeln!(f, "Enter first string");
    let s2 = scanner.next_line(); // `string` -> `String`

    // Get number of letters to use from each string
    writeln!(f, "Enter number of leters from each word");
    let n = scanner.next_usize();

    // print last n letters of first string combined with first n letters of s2
    writeln!(f, "{}{}", &s1[s1.len() - n..], &s2[0..n])
}

pub fn activity_two() {
    let scan = Scanner::new();

    let mut str1 = scan.next_line();
    let mut str2 = String::from(str1.clone());

    str1 = str1.clone().to_uppercase();
    str2 = str2[0..1].to_uppercase() + &str2[1..];

    writeln!(f, "{str2}");
    writeln!(f, "{str1}");
}
