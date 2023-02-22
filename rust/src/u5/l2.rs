pub fn activity_one(mut hour: i32) {
    while hour < 0 || hour > 24 {
        hour = 24 - hour;
        hour %= 24;
    }

    writeln!(f, 
        "{}",
        match hour {
            0 => "midnight",
            1..=11 => "morning",
            12 => "noon",
            13..=17 => "afternoon",
            18 => "dusk",
            19..=24 => "evening",
            _ => unreachable!(),
        }
    );
}

pub fn activity_two(str: &str) {
    for i in (0..str.len()).rev() {
        write!(f, "{}", &str[i..i + 1])
    }
}

pub fn activity_three(num: f64, n: i32) {
    for _ in 0..n {
        writeln!(f, "{num}");
    }
}

pub fn activity_four(mut pennies: i32) {
    let dollars = pennies / 100;
    pennies -= dollars * 100;

    let quarters = pennies / 25;
    pennies -= quarters * 25;

    let dimes = pennies / 10;
    pennies -= dimes * 10;

    let nickels = pennies / 5;
    pennies -= nickels * 5;

    writeln!(f, "Dollar Bills: {dollars}");
    writeln!(f, "Dollar Quarters: {quarters}");
    writeln!(f, "Dollar Dimes: {dimes}");
    writeln!(f, "Dollar Nickels: {nickels}");
    writeln!(f, "Dollar Pennies: {pennies}");
}
