package shapes;

public class RegularPolygon extends java.lang.Object {
    private double length = 1.0f;
    private int sides = 3;

    public RegularPolygon() {
        // don't do anything
    }

    public RegularPolygon(double length) {
        this.length = length;
    }

    public RegularPolygon(int sides) {
        this.sides = sides;
    }

    public RegularPolygon(int sides, double length) {
        this.sides = sides;
        this.length = length;
    }

    public void addSides() {
        this.sides += 1;
    }

    public void addSides(int amount) {
        this.sides += amount;
    }

    public double getArea() {
        final double apothem = (this.length) / (2 * Math.tan(Math.PI / this.sides));

        return (apothem * this.getPerimeter()) / 2;
    }

    public double getPerimeter() {
        return this.sides * this.length;
    }

    public double getSideLength() {
        return this.length;
    }

    public int getNumSides() {
        return this.sides;
    }

    public void setNumSides(int sides) {
        this.sides = sides;
    }

    public void setSideLength(double length) {
        this.length = length;
    }

    public boolean equals(RegularPolygon obj) {
        return obj.getSideLength() == this.getSideLength() && obj.getNumSides() == this.getNumSides();
    }

    public String toString() {
        String verb = this.sides + " sided polygon";

        switch (this.sides) {
            case 3:
                verb = "triangle";
                break;
            case 4:
                verb = "square";
                break;
            case 5:
                verb = "pentagon";
                break;
            case 6:
                verb = "hexagon";
                break;
            case 7:
                verb = "septgon";
                break;
            case 8:
                verb = "octagon";
                break;
            case 9:
                verb = "nonagon";
                break;
            case 10:
                verb = "decagon";
                break;
        }

        return verb + " with side length " + this.length;
    }
}
