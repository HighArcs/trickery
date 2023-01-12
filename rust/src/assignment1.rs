use crate::tools::Scanner;

pub fn assignment() {
    let s = Scanner::new();

    println!("Please enter the course name.");
    let name = s.next_line();

    println!("Please enter the average time spent in a week for this course in minutes.");

    let mut time = s.next_i32();

    println!("Please enter the homework grades for this course.");

    let homework1 = s.next_i32();
    let homework2 = s.next_i32();
    let homework3 = s.next_i32();
    let homework4 = s.next_i32();

    println!("Please enter the quiz grades for this course.");

    let quiz1 = s.next_double();
    let quiz2 = s.next_double();

    println!("Please enter the final exam grade for this course.");

    let exam = s.next_double();

    println!("Course name: {name}");

    let minutes = time % 60;
    let mut hours = 0;

    while time > 60 {
        time -= 60;
        hours += 1;
    }

    let homework_average = ((homework1 + homework2 + homework3 + homework4) as f64) / 4.0;
    let quiz_average = (quiz1 + quiz2) / 2.0;

    println!("Weekly time spent: {hours}h{minutes}");
    println!("Average homework grade: {homework_average}");
    println!("Average quiz grade: {quiz_average}");
    println!("Final exam grade: {exam}");

    let homework_weight = 0.35;
    let quiz_weight = 0.15;
    let exam_weight = 0.50;

    let average = (homework_weight * homework_average)
        + (quiz_weight * quiz_average)
        + (exam_weight * exam);

        println!("Overall grade: {}", average.round());
}
