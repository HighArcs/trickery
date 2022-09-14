import java.util.Scanner;

class Assignment1 {
    public static void main(String[] args) {
        final Scanner s = new Scanner(System.in);

        System.out.println("Please enter the course name.");

        final String n = s.nextLine();

        System.out.println("Please enter the average time spent in a week for this course in minutes.");

        int time = s.nextInt();

        System.out.println("Please enter the homework grades for this course.");

        final int homework1 = s.nextInt();
        final int homework2 = s.nextInt();
        final int homework3 = s.nextInt();
        final int homework4 = s.nextInt();

        System.out.println("Please enter the quiz grades for this course.");

        final double quiz1 = s.nextDouble();
        final double quiz2 = s.nextDouble();

        System.out.println("Please enter the final exam grade for this course.");

        final double exam = s.nextDouble();

        s.close();

        System.out.println(String.format("Course name: %s", n));

        final int minutes = time % 60;

        int hours = 0;

        while (time > 60) {
            time -= 60;
            hours += 1;
        }

        double homeworkAverage = ((double) (homework1 + homework2 + homework3 + homework4)) / 4;
        double quizAverage = (quiz1 + quiz2) / 2;

        System.out.println(String.format("Weekly time spent: %dh%d", hours, minutes));

        System.out.println("Average homework grade: " + homeworkAverage);

        System.out.println("Average quiz grade: " + quizAverage);

        System.out.println("Final exam grade: " + exam);

        double homeworkWeight = 0.35;
        double quizWeight = 0.15;
        double examWeight = 0.50;

        double average = ((homeworkWeight * homeworkAverage) + (quizWeight * quizAverage) + (exam * examWeight));

        System.out.println("Overall grade: " + Assignment1.round(average));

    }

    public static int round(double n) {
        int z = (int) n;

        if ((n - z) > 0.5) {
            return z + 1;
        }

        return z;
    }
};