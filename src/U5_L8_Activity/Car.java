// @ignore Java(536871240)
// package U5_L8_Activity;

public class Car {
    private static int uid = 0;

    private final String make;
    private final String model;
    private final int year;
    private final double mpg;
    private final int id;

    public Car(String make, String model, int year, double mpg) {
        Car.uid++;
        this.id = Car.uid;

        // year validation
        if (year > 2022) {
            year = 2022;
        }

        if (year < 1885) {
            year = 2000;
        }

        // mpg validation

        if (mpg < 0) {
            mpg = 0;
        }

        this.make = make;
        this.model = model;
        this.year = year;
        this.mpg = mpg;
    }

    public Car() {
        this("None", "None", 0, 0.0);
    }

    public String toString() {
        return "ID: " + this.id + "\nMake: " + this.make + "\nModel: " + this.model + "\nYear: " + this.year + "\nMPG: "
                + this.mpg;
    }
}
