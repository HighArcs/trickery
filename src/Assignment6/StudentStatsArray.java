import java.util.Arrays;
import java.util.Collections;

// @ignore Java(536871240)
// package Assignment6;
public class StudentStatsArray {

    // Add private final variable to hold Students array
    private final Student[] students;

    public StudentStatsArray(Student[] students) {
        this.students = students;
    }

    // Returns the average gpa of the students
    public double averageGpa() {
        double avg = 0;
        for (Student student : this.students) {
            avg += student.getGpa();
        }

        return avg / (double) this.students.length;
    }

    // Returns the gpa range of the students
    public double getGpaRange() {
        return -1;
    }

    // Returns the name of the student that has the longest length
    public String getLongestName() {
        String longest = "";
        for (Student student : this.students) {
            if (student.getName().length() > longest.length()) {
                longest = student.getName();
            }
        }

        return longest;
    }

    // Returns a count of the number freshman students
    public int getNumFreshman() {
        int count = 0;
        for (Student student : this.students) {
            if (student.getYear() == 1) {
                count++;
            }
        }

        return count;
    }

    // Returns the index of the first student with a name equal to name.
    // Returns -1 if not found
    public int search(String name) {
        for (int i = 0; i < this.students.length; i++) {
            if (this.students[i].getName().equals(name)) {
                return i;
            }
        }

        return -1;
    }

    // Returns the index of the first student with a gpa greater than or equal to
    // the gpa
    // Returns -1 if not found
    public int search(double gpa) {
        for (int i = 0; i < this.students.length; i++) {
            if (this.students[i].getGpa() >= gpa) {
                return i;
            }
        }

        return -1;
    }

    // Returns 1 if the students are sorted in ascending order by their gpa; -1 if
    // they
    // are sorted in descending order; 0 otherwise.
    public int sortStatus() {
        if (this.isSorted()) {
            return 1;
        } else if ()
    }

    private boolean isSorted() {
        if (this.students.length == 1 || this.students.length == 0)
            return true;
        for (int i = 1; i < this.students.length; i++) {
            if (this.students[i].getGpa() < this.students[i - 1].getGpa())
                return false;
        }
        return true;
    }

    private boolean isSortedDesc() {
        Student[] ck = new Student[this.students.length];
        for (int i = 0; i < this.students.length; i++) {
            ck[this.students.length - i] = this.students[i];
        }
        if (this.students.length == 1 || this.students.length == 0)
            return true;
        for (int i = 1; i < this.students.length; i++) {
            if (this.students[i].getGpa() < this.students[i - 1].getGpa())
                return false;
        }
        return true;
    }

    

    // Returns the array of students in JSON like format
    public String toString() {
        return "";
    }

}