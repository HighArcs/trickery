use crate::tools::Scanner;

use super::airplane::Airplane;

pub fn assignment_2() {
    let mut airplane1 = Airplane::default();
    let mut airplane2 = Airplane::new("AAA02".to_owned(), 15.8, 128, 30_000);

    let s = Scanner::new();
    println!(
        "Enter the details of the third airplane (call-sign, distance, bearing and altitude):"
    );
    let call_sign = s.next_line();
    let distance = s.next_double();
    let direction = s.next_i32();
    let altitude = s.next_i32();

    let mut airplane3 = Airplane::new(call_sign, distance, direction, altitude);

    print!("\n");

    println!("Initial Positions:");
    println!("\"Airplane 1: {airplane1}\"");
    println!("\"Airplane 2: {airplane2}\"");
    println!("\"Airplane 3: {airplane3}\"");

    print!("\n");

    println!("Initial Distances:");
    println!(
        "The distance between Airplane 1 and Airplane 2 is {} miles.",
        airplane1.dist_to(&airplane2)
    );
    println!(
        "The distance between Airplane 1 and Airplane 3 is {} miles.",
        airplane1.dist_to(&airplane3)
    );
    println!(
        "The distance between Airplane 2 and Airplane 3 is {} miles.",
        airplane2.dist_to(&airplane3)
    );

    print!("\n");

    println!("Initial Height Differences:");
    println!(
        "The difference in height between Airplane 1 and Airplane 2 is {} feet.",
        (airplane1.get_alt() - airplane2.get_alt()).abs()
    );
    println!(
        "The difference in height between Airplane 1 and Airplane 3 is {} feet.",
        (airplane1.get_alt() - airplane3.get_alt()).abs()
    );
    println!(
        "The difference in height between Airplane 2 and Airplane 3 is {} feet.",
        (airplane2.get_alt() - airplane3.get_alt()).abs()
    );

    print!("\n");

    airplane1.gain_alt();
    airplane1.gain_alt();
    airplane1.gain_alt();

    airplane2.lose_alt();
    airplane2.lose_alt();
    
    airplane3.lose_alt();
    airplane3.lose_alt();
    airplane3.lose_alt();
    airplane3.lose_alt();
    
    let a2a3d = airplane2.dist_to(&airplane3);
    airplane1.mov(a2a3d, 65);

    airplane2.mov(8.0, 135);

    airplane3.mov(5.0, 55);


    println!("New Positions:");
    println!("\"Airplane 1: {airplane1}\"");
    println!("\"Airplane 2: {airplane2}\"");
    println!("\"Airplane 3: {airplane3}\"");

    print!("\n");

    println!("New Distances:");
    println!(
        "The distance between Airplane 1 and Airplane 2 is {} miles.",
        airplane1.dist_to(&airplane2)
    );
    println!(
        "The distance between Airplane 1 and Airplane 3 is {} miles.",
        airplane1.dist_to(&airplane3)
    );
    println!(
        "The distance between Airplane 2 and Airplane 3 is {} miles.",
        airplane2.dist_to(&airplane3)
    );

    print!("\n");

    println!("New Height Differences:");
    println!(
        "The difference in height between Airplane 1 and Airplane 2 is {} feet.",
        (airplane1.get_alt() - airplane2.get_alt()).abs()
    );
    println!(
        "The difference in height between Airplane 1 and Airplane 3 is {} feet.",
        (airplane1.get_alt() - airplane3.get_alt()).abs()
    );
    println!(
        "The difference in height between Airplane 2 and Airplane 3 is {} feet.",
        (airplane2.get_alt() - airplane3.get_alt()).abs()
    );

    
}
